#include <stdio.h>
#include <stdlib.h>

int selection_score(int shape)
{
    switch (shape)
    {
    case 'X':
        return 1;
    case 'Y':
        return 2;
    case 'Z':
        return 3;
    }
    return 0;
}

int result(char move, char response)
{
    // A = rock, B = paper, C = scissors
    // X = rock, Y = paper, Z = scissors
    // score
    if (move == 'A')
    {
        if (response == 'X')
            return selection_score(response) + 3;
        if (response == 'Y')
            return selection_score(response) + 6;
        else
            return selection_score(response);
    }
    else if (move == 'B')
    {
        if (response == 'X')
            return selection_score(response);
        if (response == 'Y')
            return selection_score(response) + 3;
        else
            return selection_score(response) + 6;
    }
    else if (move == 'C')
    {
        if (response == 'X')
            return selection_score(response) + 6;
        if (response == 'Y')
            return selection_score(response);
        else
            return selection_score(response) + 3;
    }

    return 0;
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

    int score = 0;
    while ((read = getline(&line, &len, fp)) != -1)
    {
        char move = line[0];
        char response = line[2];

        score += result(move, response);
    }

    fclose(fp);
    if (line)
    {
        free(line);
    }

    printf("Score: %d\n", score);

    return 0;
}