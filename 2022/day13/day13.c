#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct list
{
    int is_list;
    int value;
    int elements_count;
    struct list *elements;
} list;

typedef enum compare
{
    no,
    yes,
    cont,
} compare;

char *s;

list parse()
{
    list l;
    l.elements_count = 0;
    l.value = 0;
    l.elements = malloc(sizeof(list) * 100);
    l.is_list = 1;

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

int main()
{
    FILE *fp;

    fp = fopen("input", "r");
    if (!fp)
        return -1;

    char *left = NULL, *right = NULL;
    size_t len = 0;
    int idx = 1;
    int sum = 0;

    // read inputs
    while ((getline(&left, &len, fp)) != -1)
    {
        // immediately read a second line
        getline(&right, &len, fp);

        left++;
        right++;

        s = left;
        list lleft = parse();

        s = right;
        list lright = parse();

        compare c = are_in_order(lleft, lright);
        printf("[%d] Elements are in right order? %d\n", idx, c);
        if (c == yes)
        {
            sum += idx;
        }

        left = NULL;
        getline(&left, &len, fp);
        left = NULL;
        right = NULL;
        idx++;

        // FIXME remove
        // break;
    }
    printf("Sum: %d\n", sum);

    fclose(fp);

    return 0;
}