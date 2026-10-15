/* Hand-written configuration for building libarchive's RAR and RAR5 readers with MSVC (x86_64, aarch64).
 * Only what those readers and the read core need: no compression libraries, no iconv, no crypto. */
#ifndef DEADLOADER_LIBARCHIVE_CONFIG_H
#define DEADLOADER_LIBARCHIVE_CONFIG_H

#ifndef _WIN32_WINNT
#define _WIN32_WINNT 0x0A00
#endif
#ifndef WINVER
#define WINVER 0x0A00
#endif

#define HAVE_ERRNO_H 1
#define HAVE_FCNTL_H 1
#define HAVE_INTTYPES_H 1
#define HAVE_LIMITS_H 1
#define HAVE_STDINT_H 1
#define HAVE_STDLIB_H 1
#define HAVE_STRING_H 1
#define HAVE_SYS_STAT_H 1
#define HAVE_SYS_TYPES_H 1
#define HAVE_SYS_UTIME_H 1
#define HAVE_WCHAR_H 1
#define HAVE_EILSEQ 1
#define HAVE_LOCALTIME_S 1
#define HAVE_MBRTOWC 1
#define HAVE_WCRTOMB 1
#define HAVE_WCSCPY 1
#define HAVE_WCSLEN 1
#define HAVE_WCTOMB 1
#define HAVE_WMEMCMP 1
#define HAVE_WMEMCPY 1
#define HAVE_WMEMMOVE 1

#define HAVE_DECL_SIZE_MAX 1
#define HAVE_DECL_INT32_MAX 1
#define HAVE_DECL_INT32_MIN 1
#define HAVE_DECL_UINT32_MAX 1
#define HAVE_DECL_INT64_MAX 1
#define HAVE_DECL_INT64_MIN 1
#define HAVE_DECL_UINT64_MAX 1
#define HAVE_DECL_INTMAX_MAX 1
#define HAVE_DECL_INTMAX_MIN 1
#define HAVE_DECL_UINTMAX_MAX 1

/* Types sys/types.h does not define under MSVC (values from libarchive's CMakeLists.txt). */
#define gid_t short
#define id_t short
#define mode_t unsigned short
#define pid_t int
#define ssize_t int64_t
#define uid_t short

#endif
