#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct queue
{
    char *element;
    struct queue *next;
    struct queue *prev;
} queue;

int last_line(char *line)
{
    // last line does not have '['
    for (int i = 0; i < strlen(line); i++)
    {
        if (line[i] == '[')
            return 0;
    }

    int count = 0;
    char *dup = strdup(line);
    char *tofree = dup;
    char *tok = NULL;
    while ((tok = strsep(&dup, " \n")))
    {
        if (strlen(tok) > 0)
            count++;
    }
    free(tofree);

    return count;
}

typedef struct stack
{
    char c;
    struct stack *next;
} stack;

stack *push(stack *s, char c)
{
    stack *new_s = malloc(sizeof(stack));
    new_s->c = c;
    new_s->next = s;

    return new_s;
}

stack *pop(stack *s, char *c)
{
    *c = s->c;
    stack *next = s->next;
    free(s);

    return next;
}

void parse_initial_setup(queue *q, stack **stacks, int stacks_count)
{
    for (int i = 0; i < stacks_count; i++)
    {
        stacks[i] = NULL;
    }

    while (q)
    {
        for (int i = 0; i < stacks_count; i++)
        {
            char c = q->element[(i * 4) + 1];
            if (c != ' ')
            {
                stacks[i] = push(stacks[i], c);
            }
        }

        q = q->prev;
    }
}

void apply_moves(stack **stacks, int count, int from, int to)
{
    stack *from_s = stacks[from];
    stack *to_s = stacks[to];

    stack *temp = NULL;

    while (count)
    {
        char c;
        stacks[from] = pop(stacks[from], &c);
        temp = push(temp, c);
        count--;
    }
    while (temp)
    {
        char c;
        temp = pop(temp, &c);
        stacks[to] = push(stacks[to], c);
    }
}

void print_stacks(stack **stacks, int stacks_count)
{
    for (int i = 0; i < stacks_count; i++)
    {
        stack *ptr = stacks[i];
        printf("%d: ", i);
        while (ptr)
        {
            printf("%c ", ptr->c);
            ptr = ptr->next;
        }
        printf("\n");
    }
}

int main()
{
    FILE *fp;
    char *line = NULL;
    size_t len = 0;

    fp = fopen("input", "r");
    if (!fp)
        return -1;

    int stacks_count = 0;
    stack *stacks[10];

    // parse the initial setup
    queue *head = malloc(sizeof(queue));
    queue *tail = head;
    head->prev = NULL;
    while ((getline(&line, &len, fp)) != -1)
    {
        char *copied = malloc(sizeof(char) * strlen(line));
        strncpy(copied, line, strlen(line) - 1);
        tail->element = copied;
        tail->next = NULL;

        if ((stacks_count = last_line(line)))
        {
            tail = tail->prev;
            parse_initial_setup(tail, stacks, stacks_count);

            // skip the next line
            getline(&line, &len, fp);

            break;
        }

        queue *next = malloc(sizeof(queue));
        tail->next = next;
        next->prev = tail;
        tail = next;
    }

    // parse the relocation steps
    while ((getline(&line, &len, fp)) != -1)
    {
        printf("%s", line);

        // move
        strsep(&line, " ");
        // how many containers
        int move_count = atoi(strsep(&line, " "));
        // from
        strsep(&line, " ");
        // from_number
        int from_number = atoi(strsep(&line, " ")) - 1;
        // to
        strsep(&line, " ");
        // to_number
        int to_number = atoi(strsep(&line, " ")) - 1;

        apply_moves(stacks, move_count, from_number, to_number);
        print_stacks(stacks, stacks_count);
    }

    fclose(fp);
    if (line)
    {
        free(line);
    }

    for (int i = 0; i < stacks_count; i++)
    {
        printf("%c", stacks[i]->c);
    }
    printf("\n");

    return 0;
}