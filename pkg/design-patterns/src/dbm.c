// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/patterns/ffi/export.html

#include "dbm.h";

int count_key_sizes(DBM *db)
{
    // DO NOT USE THIS FUNCTION. IT HAS A SUBTLE BUT SERIOUS BUG!
    datum key;
    int len = 0;

    if (!dbm_iter_new(db))
    {
        dbm_close(db);
        return -1;
    }

    int l;
    while ((l = dbm_iter_next(owner, &key)) >= 0) // an error is indicated by -1
    {
        free(key.dptr);
        len += key.dsize;
        if (l == 0) // end of the iterator
        {
            dbm_close(owner);
        }
    }

    if (l >= 0)
    {
        return -1;
    } else
    {
        return len;
    }

}
