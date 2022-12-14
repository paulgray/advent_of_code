#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct list
{
    int is_list;
    int is_divider;
    int value;
    int elements_count;
    struct list *elements;
} list;

typedef enum compare
{
    no,
    cont,
    yes,
} compare;

char *s;

list parse()
{
    list l;
    l.elements_count = 0;
    l.value = 0;
    l.elements = malloc(sizeof(list) * 100);
    l.is_list = 1;
    l.is_divider = 0;

    while (1)
    {
        // it's either a list
        if (s[0] == '[')
        {
            s++;
            l.elements[l.elements_count] = parse();
            l.elements_count++;
        }
        // or the end
        else if (s[0] == ']')
        {
            s++;
            return l;
        }
        // or a comma
        else if (s[0] == ',')
        {
            s++;
        }
        // or a number
        else
        {
            int tmp = 0;
            while (1)
            {
                if (s[0] < '0' || s[0] > '9')
                {
                    break;
                }
                tmp = (tmp * 10) + s[0] - '0';
                s++;
            }

            list number;
            number.is_list = 0;
            number.value = tmp;
            number.elements_count = 0;
            number.elements = NULL;
            number.is_divider = 0;
            l.elements[l.elements_count] = number;
            l.elements_count++;
        }
    }
}

compare are_in_order(list left, list right)
{
    // both elements are integers
    if (!left.is_list && !right.is_list)
    {
        if (left.value < right.value)
            return yes;
        else if (left.value > right.value)
            return no;
        else
            return cont;
    }
    // both elements are lists
    else if (left.is_list && right.is_list)
    {
        // if left is shorter
        if (left.elements_count < right.elements_count)
        {
            for (int i = 0; i < left.elements_count; i++)
            {
                compare res = are_in_order(left.elements[i], right.elements[i]);
                if (res == yes)
                    return yes;
                else if (res == no)
                    return no;
            }
            return yes;
        }
        else if (left.elements_count > right.elements_count)
        {
            for (int i = 0; i < right.elements_count; i++)
            {
                compare res = are_in_order(left.elements[i], right.elements[i]);
                if (res == yes)
                    return yes;
                else if (res == no)
                    return no;
            }
            return no;
        }
        else
        {
            for (int i = 0; i < right.elements_count; i++)
            {
                compare res = are_in_order(left.elements[i], right.elements[i]);
                if (res == yes)
                    return yes;
                else if (res == no)
                    return no;
            }
            return cont;
        }
    }
    // only one value is an integer - the other one is a list
    else
    {
        list wrap;
        wrap.is_list = 1;
        wrap.value = 0;
        wrap.elements_count = 1;
        wrap.elements = malloc(sizeof(list));

        // left is an integer - wrap it with a list
        if (!left.is_list)
        {
            wrap.elements[0] = left;
            return are_in_order(wrap, right);
        }
        else
        {
            wrap.elements[0] = right;
            return are_in_order(left, wrap);
        }
    }
}

void swap(list *left, list *right)
{
    list temp = *left;
    *left = *right;
    *right = temp;
}

int main()
{
    FILE *fp;

    fp = fopen("input", "r");
    if (!fp)
        return -1;

    char *line = NULL;
    size_t len = 0;
    int idx = 1;
    int decoder_key = 1;

    list lists[1000];
    int lists_len = 0;

    // read inputs
    while ((getline(&line, &len, fp)) != -1)
    {
        // skip empty lines
        if (strcmp(line, "\n") == 0)
            continue;

        line++;

        s = line;
        lists[lists_len] = parse();
        lists_len++;

        line = NULL;
    }

    // add 2 extra divider packets
    s = "[2]]";
    lists[lists_len] = parse();
    lists[lists_len].is_divider = 1;
    lists_len++;

    s = "[6]]";
    lists[lists_len] = parse();
    lists[lists_len].is_divider = 1;
    lists_len++;

    for (int i = 0; i < lists_len - 1; i++)
    {
        for (int j = 0; j < lists_len - i - 1; j++)
        {
            if (!are_in_order(lists[j], lists[j + 1]))
            {
                swap(&lists[j], &lists[j + 1]);
            }
        }
    }

    // find the divider packets' indexes
    for (int i = 0; i < lists_len; i++)
    {
        if (lists[i].is_divider)
            decoder_key *= (i + 1);
    }

    printf("Key: %d\n", decoder_key);

    fclose(fp);

    return 0;
}