#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct coord
{
    int x;
    int y;
} coord;

void find_way(char **lines, int rows, int columns, int steps, coord c, coord e, int costs[1000][1000])
{
    // stop if we are at the target
    if (c.x == e.x && c.y == e.y)
        return;

    // check if we want to go up
    if (c.y > 0)
    {
        // we can reach it AND it's cheaper to go there
        if (lines[c.y - 1][c.x] <= lines[c.y][c.x] + 1 && costs[c.y - 1][c.x] > steps + 1)
        {
            costs[c.y - 1][c.x] = steps + 1;
            c.y--;
            find_way(lines, rows, columns, steps + 1, c, e, costs);
            c.y++;
        }
    }

    // check if we want to go down
    if (c.y < rows)
    {
        // we can reach it AND it's cheaper to go there
        if (lines[c.y + 1][c.x] <= lines[c.y][c.x] + 1 && costs[c.y + 1][c.x] > steps + 1)
        {
            costs[c.y + 1][c.x] = steps + 1;
            c.y++;
            find_way(lines, rows, columns, steps + 1, c, e, costs);
            c.y--;
        }
    }

    // check if we want to go left
    if (c.x > 0)
    {
        // we can reach it AND it's cheaper to go there
        if (lines[c.y][c.x - 1] <= lines[c.y][c.x] + 1 && costs[c.y][c.x - 1] > steps + 1)
        {
            costs[c.y][c.x - 1] = steps + 1;
            c.x--;
            find_way(lines, rows, columns, steps + 1, c, e, costs);
            c.x++;
        }
    }

    // check if we want to go right
    if (c.x < columns)
    {
        // we can reach it AND it's cheaper to go there
        if (lines[c.y][c.x + 1] <= lines[c.y][c.x] + 1 && costs[c.y][c.x + 1] > steps + 1)
        {
            costs[c.y][c.x + 1] = steps + 1;
            c.x++;
            find_way(lines, rows, columns, steps + 1, c, e, costs);
            c.x--;
        }
    }
}

int main()
{
    FILE *fp;

    fp = fopen("input", "r");
    if (!fp)
        return -1;

    char *lines[10000];
    int rows = 0;
    int columns = 0;
    size_t len = 0;

    // read all the lines first
    while ((getline(&lines[rows++], &len, fp)) != -1)
    {
    }
    columns = strlen(lines[0]) - 1;
    rows--;

    // find the starting and ending positions
    coord s, e;
    int costs[1000][1000];
    for (int i = 0; i < rows; i++)
    {
        for (int j = 0; j < columns; j++)
        {
            costs[i][j] = 999999;
            if (lines[i][j] == 'S')
            {
                s.x = j;
                s.y = i;
                lines[i][j] = 'a';
            }
            else if (lines[i][j] == 'E')
            {
                e.x = j;
                e.y = i;
                lines[i][j] = 'z';
            }
        }
    }

    // start moving
    find_way(lines, rows, columns, 0, s, e, costs);
    printf("Cost to get to (%d, %d) = %d\n", e.x, e.y, costs[e.y][e.x]);

    fclose(fp);

    return 0;
}