//! The images the catalog has handed out ids for, and what each id may do.
//!
//! In memory only, bounded: when it is full the oldest ids are forgotten, and the UI gets them again with the next
//! page or mod page it asks for. A forgotten id answers "unknown", never something else.

use crate::id::ImageId;
use deadloader_net::{Initiator, Purpose};
use std::collections::{HashMap, VecDeque};

/// Ids kept at most: ~4 MB at the worst, a full scroll through every Deadlock mod needs ~10 000.
pub(crate) const CAPACITY: usize = 20_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Kind {
    /// A catalog card's picture (220 or 530 px).
    Thumbnail,
    /// A mod page's gallery picture (530 px).
    Gallery,
    /// A gallery picture at full size, on demand.
    Original,
    /// A picture in a description; needs a click, and consent for hosts not allowed yet (decisions 29, 122).
    Description,
}

impl Kind {
    pub(crate) fn tag(self) -> &'static str {
        match self {
            Self::Thumbnail => "thumbnail",
            Self::Gallery => "gallery",
            Self::Original => "original",
            Self::Description => "description",
        }
    }

    pub(crate) fn purpose(self) -> Purpose {
        match self {
            Self::Description => Purpose::DescriptionImage,
            _ => Purpose::Thumbnail,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Entry {
    pub kind: Kind,
    pub url: String,
    /// Lowercase host of `url`.
    pub host: String,
    pub initiator: Initiator,
    /// Description images: the user clicked it (or "always load" applies), so it may be fetched.
    pub activated: bool,
    /// Description images: the user allowed this one image without allowing its host.
    pub once: bool,
    /// Order of first registration.
    pub seq: u64,
}

#[derive(Debug)]
pub(crate) struct Registry {
    entries: HashMap<ImageId, Entry>,
    order: VecDeque<ImageId>,
    capacity: usize,
    next_seq: u64,
}

impl Registry {
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            entries: HashMap::new(),
            order: VecDeque::new(),
            capacity: capacity.max(1),
            next_seq: 0,
        }
    }

    /// The id for `url` of this kind; a known id keeps its state.
    pub(crate) fn register(
        &mut self,
        kind: Kind,
        url: &str,
        host: &str,
        initiator: Initiator,
    ) -> ImageId {
        let id = ImageId::of(kind.tag(), url);
        if self.entries.contains_key(&id) {
            return id;
        }
        self.next_seq += 1;
        self.entries.insert(
            id,
            Entry {
                kind,
                url: url.to_owned(),
                host: host.to_owned(),
                initiator,
                activated: false,
                once: false,
                seq: self.next_seq,
            },
        );
        self.order.push_back(id);
        while self.order.len() > self.capacity {
            if let Some(old) = self.order.pop_front() {
                self.entries.remove(&old);
            }
        }
        id
    }

    pub(crate) fn get(&self, id: &ImageId) -> Option<&Entry> {
        self.entries.get(id)
    }

    pub(crate) fn get_mut(&mut self, id: &ImageId) -> Option<&mut Entry> {
        self.entries.get_mut(id)
    }

    /// Description images from `host`, oldest first.
    pub(crate) fn descriptions_from(&mut self, host: &str) -> Vec<(ImageId, &mut Entry)> {
        let mut found: Vec<_> = self
            .entries
            .iter_mut()
            .filter(|(_, e)| e.kind == Kind::Description && e.host == host)
            .map(|(id, e)| (*id, e))
            .collect();
        found.sort_by_key(|(_, e)| e.seq);
        found
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.entries.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add(r: &mut Registry, n: u32) -> ImageId {
        r.register(
            Kind::Thumbnail,
            &format!("https://images.gamebanana.com/{n}.jpg"),
            "images.gamebanana.com",
            Initiator::Auto,
        )
    }

    #[test]
    fn registering_again_keeps_the_id_and_the_state() {
        let mut r = Registry::new(10);
        let url = "https://i.imgur.com/a.png";
        let id = r.register(Kind::Description, url, "i.imgur.com", Initiator::User);
        r.get_mut(&id).expect("entry").activated = true;
        assert_eq!(
            r.register(Kind::Description, url, "i.imgur.com", Initiator::User),
            id
        );
        assert!(r.get(&id).expect("entry").activated);
        assert_eq!(r.len(), 1);
    }

    #[test]
    fn the_oldest_ids_are_forgotten_beyond_the_capacity() {
        let mut r = Registry::new(3);
        let ids: Vec<_> = (0..5).map(|n| add(&mut r, n)).collect();
        assert_eq!(r.len(), 3);
        assert!(r.get(&ids[0]).is_none());
        assert!(r.get(&ids[1]).is_none());
        assert!(ids[2..].iter().all(|id| r.get(id).is_some()));
    }

    #[test]
    fn description_images_of_a_host_come_in_registration_order() {
        let mut r = Registry::new(10);
        let b = r.register(
            Kind::Description,
            "https://h.example/b",
            "h.example",
            Initiator::User,
        );
        let a = r.register(
            Kind::Description,
            "https://h.example/a",
            "h.example",
            Initiator::User,
        );
        r.register(
            Kind::Description,
            "https://o.example/c",
            "o.example",
            Initiator::User,
        );
        r.register(
            Kind::Gallery,
            "https://h.example/d",
            "h.example",
            Initiator::Auto,
        );
        let found: Vec<_> = r
            .descriptions_from("h.example")
            .into_iter()
            .map(|(id, _)| id)
            .collect();
        assert_eq!(found, [b, a]);
    }
}
