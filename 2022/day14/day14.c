#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_SIZE 5000

void print_map(char **map, int min_x, int max_x, int min_y, int max_y)
{
    for (int i = min_y; i <= max_y; i++)
    {
        printf("%02d\t", i - min_y);
        for (int j = min_x - 100; j <= max_x + 100; j++)
        {
            printf("%c", map[i][j]);
        }
        printf("\n");
    }
}

void draw_line(char **map, int x1, int y1, int x2, int y2)
{
    // we don't have a start
    if (x1 == -1)
        return;

    int start_x = x1 > x2 ? x2 : x1;
    int end_x = x1 > x2 ? x1 : x2;
    int start_y = y1 > y2 ? y2 : y1;
    int end_y = y1 > y2 ? y1 : y2;

    for (int x = start_x; x <= end_x; x++)
        for (int y = start_y; y <= end_y; y++)
            map[y][x] = '#';
}

int main()
{
    FILE *fp;

    fp = fopen("input", "r");
    if (!fp)
        return -1;

    char *line = NULL;
    size_t len = 0;

    char **map = malloc(sizeof(char *) * MAX_SIZE);
    for (int i = 0; i < MAX_SIZE; i++)
        map[i] = malloc(sizeof(char) * MAX_SIZE);

    int min_x = 9999, min_y = 0, max_x = 0, max_y = 0;
    int shift = 500;

    // create an empty map
    for (int i = 0; i < MAX_SIZE; i++)
    {
        for (int j = 0; j < MAX_SIZE; j++)
        {
            map[i][j] = '.';
        }
    }

    // read inputs
    while ((getline(&line, &len, fp)) != -1)
    {
        // coords of the last point that we should draw a line to
        int prev_x = -1, prev_y = -1;

        // split by '->'
        char *pair = strsep(&line, " ");
        while (pair != NULL)
        {
            int x = atoi(strsep(&pair, ",")) + shift;
            int y = atoi(pair);
            draw_line(map, prev_x, prev_y, x, y);

            if (x > max_x)
                max_x = x;
            if (x < min_x)
                min_x = x;
            if (y > max_y)
                max_y = y;
            if (y < min_y)
                min_y = y;

            prev_x = x;
            prev_y = y;

            // now - skip the arrow
            strsep(&line, " ");
            pair = strsep(&line, " ");
        }

        line = NULL;
    }

    max_y += 2;
    draw_line(map, 0, max_y, MAX_SIZE - 1, max_y);

    // print map
    print_map(map, min_x, max_x, min_y, max_y);

    // start pouring down sand
    int stop_cond = 1;
    int sand_count = 0;
    while (stop_cond)
    {
        // sand starts falling from 500,0
        int s_x = 500 + shift, s_y = 0;

        // simulate falling until it's possible
        while (1)
        {
            // there is space right under the sand
            if (map[s_y + 1][s_x] == '.')
            {
                s_y++;
            }
            // go down and left
            else if (map[s_y + 1][s_x - 1] == '.')
            {
                s_y++;
                s_x--;
            }
            // go down and right
            else if (map[s_y + 1][s_x + 1] == '.')
            {
                s_y++;
                s_x++;
            }
            // sand comes to rest
            else
            {
                map[s_y][s_x] = 'o';
                sand_count++;
                if (s_y == 0 && s_x == 500 + shift)
                    stop_cond = 0;
                break;
            }

            // but make sure we don't start falling off the map
            if (s_y > max_y)
            {
                stop_cond = 0;
                break;
            }
        }
    }

    printf("\n\n\n");
    print_map(map, min_x, max_x, min_y, max_y);

    printf("\n\nSands: %d\n", sand_count);

    fclose(fp);

    return 0;
}