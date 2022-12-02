#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>

int main()
{
    FILE *fp;
    char *line = NULL;
    size_t len = 0;
    ssize_t read;

    fp = fopen("input", "r");
    if (!fp)
        return -1;

    long long max = 0;
    long long so_far = 0;
    char *endptr;
    while ((read = getline(&line, &len, fp)) != -1)
    {
        long long val = strtoimax(line, &endptr, 10);

        if (val > 0)
        {
            so_far += val;
        }
        else if (so_far > max)
        {
            max = so_far;
            so_far = 0;
        }
        else
        {
            so_far = 0;
        }
    }

    fclose(fp);
    if (line)
    {
        free(line);
    }

    printf("Max: %lld\n", max);

    return 0;
}