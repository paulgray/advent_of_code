#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int is_visible(char **grid, int rows, int columns, int y, int x)
{
    // we are on the edge
    if (x == 0 || y == 0 || x == columns - 1 || y == rows - 1)
        return 1;

    // we are in the interior, need to check all 4 directions
    char height = grid[y][x];
    int visilbe_counter = 4;

    // from the left
    for (int i = 0; i < x; i++)
    {
        if (grid[y][i] >= height)
        {
            visilbe_counter--;
            break;
        }
    }

    // from the right
    for (int i = x + 1; i < columns; i++)
    {
        if (grid[y][i] >= height)
        {
            visilbe_counter--;
            break;
        }
    }

    // from the top
    for (int i = 0; i < y; i++)
    {
        if (grid[i][x] >= height)
        {
            visilbe_counter--;
            break;
        }
    }

    // from the bottom
    for (int i = y + 1; i < rows; i++)
    {
        if (grid[i][x] >= height)
        {
            visilbe_counter--;
            break;
        }
    }

    return visilbe_counter;
}

int main()
{
    FILE *fp;

    fp = fopen("input", "r");
    if (!fp)
        return -1;

    char *lines[10000];
    size_t len = 0;
    int rows = 0;
    while ((getline(&lines[rows++], &len, fp)) != -1)
    {
        // read all the lines first
    }
    rows--;
    int columns = strlen(lines[0]) - 1;

    // count visible trees
    int visible = 0;
    for (int row = 0; row < rows; row++)
    {
        for (int column = 0; column < columns; column++)
        {
            if (is_visible(lines, rows, columns, row, column))
                visible++;
        }
    }

    printf("Visible: %d\n", visible);

    fclose(fp);

    return 0;
}