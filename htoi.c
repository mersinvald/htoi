#include <stdio.h>

static const char hextable[] = {
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1, 0,1,2,3,4,5,6,7,8,9,-1,-1,-1,-1,-1,-1,-1,10,11,12,13,14,15,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,10,11,12,13,14,15,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1
};

long long htoi_c_table(char *s) {
    int offset = (s[1] == 'x' || s[1] == 'X') ? 2 : 0;
    long long result = 0;
    for (char *temp = s + offset; *temp; temp++)
    {
        signed char digit = hextable[(size_t)*temp];
        result = (result << 4) + digit;
    }
    return result;
}

long long htoi_c(char *s)
{
    int offset = (s[0] == '0' && (s[1] == 'x' || s[1] == 'X')) ? 2 : 0;
    long long result = 0;
    for (char *temp = s + offset; *temp; temp++)
    {
        signed char digit = ((*temp >= '0') && (*temp <= '9')) ?
            (*temp - '0') :
            (((*temp >= 'A') && (*temp <= 'F')) ?
                (*temp - 'A' + 10) :
                (((*temp >= 'a') && (*temp <= 'f')) ?
                    (*temp - 'a' + 10) :
                    -1));
        if (digit==-1)
        {
            return -1;
        }
        result = (result << 4) + digit;
    }
    return result;
}