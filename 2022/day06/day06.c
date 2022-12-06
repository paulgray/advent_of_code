#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int are_all_different(char *buffer, int len)
{
    for (int i = 0; i < len; i++)
    {
        for (int j = i + 1; j < len; j++)
        {
            if (buffer[i] == buffer[j])
                return 0;
        }
    }

    return 1;
}

int find_marker(char *signal)
{
    int pos;

    for (pos = 0; pos < strlen(signal) - 3; pos++)
    {
        if (are_all_different(signal + pos, 4))
            break;
    }

    return pos + 4;
}

int main()
{
    FILE *fp;
    char *line = NULL;
    size_t len = 0;

    fp = fopen("input", "r");
    if (!fp)
        return -1;

    while ((getline(&line, &len, fp)) != -1)
    {
        int pos = find_marker(line);
        printf("%d\n", pos);
    }

    fclose(fp);
    if (line)
    {
        free(line);
    }

    return 0;
}
