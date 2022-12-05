#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int cmpfunc(const void *a, const void *b)
{
    return (*(char *)a - *(char *)b);
}

int value(char c)
{
    if (c >= 'a' && c <= 'z')
    {
        return c - 'a' + 1;
    }
    else if (c >= 'A' && c <= 'Z')
    {
        return c - 'A' + 27;
    }
    return -1;
}

int duplicates_search(char *left, char *right, int len)
{
    for (int i = 0; i < len; i++)
    {
        // the arrays are sorted, so we can stop searching
        // as soon as the compared element is
        for (int j = 0; j < len; j++)
        {
            if (left[i] == right[j])
            {
                return value(left[i]);
            }
            else if (left[i] < right[j])
            {
                break;
            }
        }
    }
    return -1;
}

int main()
{
    FILE *fp;
    char *line = NULL;
    size_t len = 0;
    ssize_t read;

    int sum = 0;
    fp = fopen("input", "r");
    if (!fp)
        return -1;

    while ((read = getline(&line, &len, fp)) != -1)
    {
        int slen = (strlen(line) - 1) / 2;

        // sort both halves of the string
        qsort(line, slen, sizeof(char), cmpfunc);
        qsort(line + slen, slen, sizeof(char), cmpfunc);

        // look for duplicates
        sum += duplicates_search(line, line + slen, slen);
    }

    fclose(fp);
    if (line)
    {
        free(line);
    }

    printf("Priorities: %d\n", sum);

    return 0;
}