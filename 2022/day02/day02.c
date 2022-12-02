#include <stdio.h>
#include <stdlib.h>

int selection_score(int shape)
{
    switch (shape)
    {
    case 'X':
    case 'A':
        return 1;
    case 'Y':
    case 'B':
        return 2;
    case 'Z':
    case 'C':
        return 3;
    }
    return 0;
}

int result(char move, char response)
{
    // A = rock, B = paper, C = scissors
    // X = rock, Y = paper, Z = scissors
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

int result2(char move, char expected_result)
{
    // A = rock, B = paper, C = scissors
    // X = lose, Y = draw,  Z = win
    if (move == 'A')
    {
        if (expected_result == 'X')
            return selection_score('C');
        if (expected_result == 'Y')
            return selection_score('A') + 3;
        else
            return selection_score('B') + 6;
    }
    else if (move == 'B')
    {
        if (expected_result == 'X')
            return selection_score('A');
        if (expected_result == 'Y')
            return selection_score('B') + 3;
        else
            return selection_score('C') + 6;
    }
    else if (move == 'C')
    {
        if (expected_result == 'X')
            return selection_score('B');
        if (expected_result == 'Y')
            return selection_score('C') + 3;
        else
            return selection_score('A') + 6;
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

        score += result2(move, response);
    }

    fclose(fp);
    if (line)
    {
        free(line);
    }

    printf("Score: %d\n", score);

    return 0;
}