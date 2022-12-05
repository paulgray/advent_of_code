#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct range
{
    int left;
    int right;
} range;

range parse_range(char *s)
{
    range r;

    char *subs = strsep(&s, "-");
    r.left = atoi(subs);

    subs = strsep(&s, "-");
    r.right = atoi(subs);

    return r;
}

int does_contain(range r1, range r2)
{
    // check if range1 contains entire range2
    if (r1.left <= r2.left && r1.right >= r2.right)
        return 1;

    // or the other way around
    if (r2.left <= r1.left && r2.right >= r1.right)
        return 1;

    return 0;
}

int main()
{
    FILE *fp;
    char *line = NULL;
    size_t len = 0;

    int overlaps = 0;
    fp = fopen("input", "r");
    if (!fp)
        return -1;

    while ((getline(&line, &len, fp)) != -1)
    {
        char *left, *right;
        line[strlen(line) - 1] = '\0';

        left = strsep(&line, ",");
        right = strsep(&line, ",");

        if (does_contain(parse_range(left), parse_range(right)))
            overlaps++;
    }

    fclose(fp);
    if (line)
        free(line);

    printf("Overlaps: %d\n", overlaps);

    return 0;
}