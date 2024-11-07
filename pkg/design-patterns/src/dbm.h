// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/patterns/ffi/export.html

struct DBM;
typedef struct { void *dptr, size_t dsize } datum;

int dbm_clearerr(DBM *);
void dbm_close(DBM *);
int dbm_delete(DBM *, datum);
int dbm_error(DBM *);
datum dbm_fetch(DBM *, datum);
datum dbm_firstkey(DBM *);
datum dbm_nextkey(DBM *);
DBM *dbm_open(const char *, int, mode_t);
int dbm_store(DBM *, datum, datum, int);
