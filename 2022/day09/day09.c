#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct pair
{
    int x;
    int y;
} pair;

typedef struct snake
{
    pair head;
    pair tail;

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

void follow_tail(snake *s)
{
    // do nothing if we are in the same place, or touching
    if (distance(s->head, s->tail) <= 1)
        return;

    // otherwise we need to move the tail in the direction of head
    // move horizontally
    // if (s->head.y == s->tail.y)
    {
        if (s->head.x > s->tail.x)
        {
            s->tail.x++;
        }
        else if (s->head.x < s->tail.x)
        {
            s->tail.x--;
        }
    }

    // if (s->head.x == s->tail.x)
    {
        if (s->head.y > s->tail.y)
        {
            s->tail.y++;
        }
        else if (s->head.y < s->tail.y)
        {
            s->tail.y--;
        }
    }
}

void update_visited(snake *s)
{
    // see if we already visited this point
    for (int i = 0; i < s->visited_count; i++)
    {
        if (s->tail.x == s->tail_visited[i].x &&
            s->tail.y == s->tail_visited[i].y)
        {
            return;
        }
    }

    // not really, let's append this
    s->tail_visited[s->visited_count] = s->tail;
    s->visited_count++;
}

void move(snake *s, char direction, int steps)
{
    // printf("Moving head from (%d, %d) %d fields to %c\n", s->head.x, s->head.y, steps, direction);
    for (int i = 0; i < steps; i++)
    {
        // move head first
        move_point(&s->head, direction);

        // try to follow with the tail
        follow_tail(s);

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
    s.head = start;
    s.tail = start;
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