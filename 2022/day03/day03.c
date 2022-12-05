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

// this would be 100x more elegant with sets/hashmaps
int badge_search(char *elf1, int len1, char *elf2, int len2, char *elf3, int len3)
{
    // each elf backpack is sorted
    // the character we're looking for must appear in _all_ lines
    for (int i = 0; i < len1; i++)
    {
        for (int j = 0; j < len2; j++)
        {
            if (elf1[i] == elf2[j])
            {
                // now we need to see if the element is also present
                // in the third elf rucksack
                for (int k = 0; k < len3; k++)
                {
                    if (elf1[i] == elf3[k])
                    {
                        return value(elf1[i]);
                    }
                    else if (elf1[i] < elf3[k])
                    {
                        break;
                    }
                }
            }
            else if (elf1[i] < elf2[j])
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
    char *line1 = NULL;
    char *line2 = NULL;
    char *line3 = NULL;
    size_t len1 = 0, len2 = 0, len3 = 0;

    int sum = 0;
    fp = fopen("input", "r");
    if (!fp)
        return -1;

    while ((getline(&line1, &len1, fp)) != -1 &&
           (getline(&line2, &len2, fp)) != -1 &&
           (getline(&line3, &len3, fp)) != -1)
    {
        // sort each of the strings
        qsort(line1, strlen(line1) - 1, sizeof(char), cmpfunc);
        qsort(line2, strlen(line2) - 1, sizeof(char), cmpfunc);
        qsort(line3, strlen(line3) - 1, sizeof(char), cmpfunc);

        // look for duplicates
        sum += badge_search(line1, len1, line2, len2, line3, len3);
    }

    fclose(fp);
    if (line1)
        free(line1);
    if (line2)
        free(line2);
    if (line3)
        free(line3);

    printf("Priorities: %d\n", sum);

    return 0;
}