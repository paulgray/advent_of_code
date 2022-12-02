#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>

void add_calories(long long *calories, long long elf)
{
    // assume it's always sorted
    if (elf > calories[0])
    {
        calories[2] = calories[1];
        calories[1] = calories[0];
        calories[0] = elf;
    }
    else if (elf > calories[1])
    {
        calories[2] = calories[1];
        calories[1] = elf;
    }
    else if (elf > calories[2])
    {
        calories[2] = elf;
    }
}

int main()
{
    FILE *fp;
    char *line = NULL;
    size_t len = 0;
    ssize_t read;

    fp = fopen("input", "r");
    if (!fp)
        return -1;

    long long so_far = 0;
    char *endptr;
    long long calories[3] = {0, 0, 0};
    while ((read = getline(&line, &len, fp)) != -1)
    {
        long long val = strtoimax(line, &endptr, 10);

        if (val > 0)
        {
            so_far += val;
        }
        else
        {
            add_calories(calories, so_far);
            so_far = 0;
        }
    }

    fclose(fp);
    if (line)
    {
        free(line);
    }

    printf("top 3: %lld, %lld, %lld, %lld\n", calories[0], calories[1], calories[2], calories[0] + calories[1] + calories[2]);

    return 0;
}