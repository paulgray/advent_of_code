#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define BODY_SIZE 10

typedef struct pair
{
    int x;
    int y;
} pair;

typedef struct snake
{
    pair body[BODY_SIZE];

    pair tail_visited[100000];
    int visited_count;
} snake;

void move_point(pair *p, char direction)
{
    if (direction == 'U')
    {
        p->y++;
    }
    else if (direction == 'D')
    {
        p->y--;
    }
    else if (direction == 'L')
    {
        p->x--;
    }
    else if (direction == 'R')
    {
        p->x++;
    }
}

int distance(pair a, pair b)
{
    int x = abs(a.x - b.x);
    int y = abs(a.y - b.y);

    if (x == 0 && y == 0)
        return 0;
    if ((x == 1 || y == 1) && x < 2 && y < 2)
        return 1;

    // whatever, it's always at least 2
    return 2;
}

void follow_head(snake *s)
{
    // for each element of the body of the snake, follow the previous one
    for (int i = 1; i < BODY_SIZE; i++)
    {
        pair to_follow = s->body[i - 1];

        // do nothing if we are in the same place, or touching
        if (distance(to_follow, s->body[i]) <= 1)
            return;

        // otherwise we need to move the body in the direction of head
        if (to_follow.x > s->body[i].x)
        {
            s->body[i].x++;
        }
        else if (to_follow.x < s->body[i].x)
        {
            s->body[i].x--;
        }

        if (to_follow.y > s->body[i].y)
        {
            s->body[i].y++;
        }
        else if (to_follow.y < s->body[i].y)
        {
            s->body[i].y--;
        }
    }
}

void update_visited(snake *s)
{
    // see if we already visited this point
    for (int i = 0; i < s->visited_count; i++)
    {
        if (s->body[BODY_SIZE - 1].x == s->tail_visited[i].x &&
            s->body[BODY_SIZE - 1].y == s->tail_visited[i].y)
        {
            return;
        }
    }

    // not really, let's append this
    s->tail_visited[s->visited_count] = s->body[BODY_SIZE - 1];
    s->visited_count++;
}

void move(snake *s, char direction, int steps)
{
    printf("Moving head from (%d, %d) %d fields to %c\n", s->body[0].x, s->body[0].y, steps, direction);
    for (int i = 0; i < steps; i++)
    {
        // move head first
        move_point(&s->body[0], direction);

        // try to follow with the tail
        follow_head(s);

        // update visited points
        update_visited(s);
    }
}

int main()
{
    FILE *fp;

    fp = fopen("input", "r");
    if (!fp)
        return -1;

    char *line = NULL;
    size_t len = 0;

    snake s;
    pair start = {0, 0};
    for (int i = 0; i < BODY_SIZE; i++)
        s.body[i] = start;
    s.visited_count = 0;
    s.tail_visited[s.visited_count++] = start;
    while ((getline(&line, &len, fp)) != -1)
    {
        char dir = strsep(&line, " ")[0];
        int steps = atoi(line);

        move(&s, dir, steps);
    }

    printf("Visited: %d\n", s.visited_count);

    fclose(fp);

    return 0;
}