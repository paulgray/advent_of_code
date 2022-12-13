#include <stdio.h>
#include <stdlib.h>
#include <string.h>

long long addition(long long a, long long b)
{
    return a + b;
}

long long multiplication(long long a, long long b)
{
    return a * b;
}

long long square(long long a, long long ignore)
{
    return a * a;
}

typedef struct monkey
{
    long long items[100];
    int items_count;

    long long (*operation)(long long, long long);
    long long operation_arg;

    long long divisible_by_test;
    int if_true;
    int if_false;

    long long inspected;
} monkey;

int cmp(const void *a, const void *b)
{
    if ((*(long long *)b) > *(long long *)a)
    {
        return 1;
    }
    else if ((*(long long *)b) == *(long long *)a)
    {
        return 0;
    }
    else
    {
        return -1;
    }
}

void simulate(monkey *monkeys, int monkey_count)
{
    for (int round = 0; round < 10000; round++)
    {
        // for every monkey
        for (int monkey_no = 0; monkey_no < monkey_count; monkey_no++)
        {
            // inspect each item
            monkeys[monkey_no].inspected += monkeys[monkey_no].items_count;
            for (int item_no = 0; item_no < monkeys[monkey_no].items_count; item_no++)
            {
                monkeys[monkey_no].items[item_no] = monkeys[monkey_no].operation(monkeys[monkey_no].items[item_no], monkeys[monkey_no].operation_arg) % 96577;
                // monkeys[monkey_no].items[item_no] /= 3;

                int target_monkey_id = 0;
                if ((monkeys[monkey_no].items[item_no] % monkeys[monkey_no].divisible_by_test) == 0)
                {
                    target_monkey_id = monkeys[monkey_no].if_true;
                }
                else
                {
                    target_monkey_id = monkeys[monkey_no].if_false;
                }
                int target_monkey_item_count = monkeys[target_monkey_id].items_count;
                monkeys[target_monkey_id].items[target_monkey_item_count] = monkeys[monkey_no].items[item_no] % 96577;
                monkeys[target_monkey_id].items_count++;
                monkeys[monkey_no].items[item_no] = 0;
            }
            monkeys[monkey_no].items_count = 0;
        }
    }

    // find 2 most active monkeys
    long long inspected[100];
    for (int i = 0; i < monkey_count; i++)
    {
        inspected[i] = monkeys[i].inspected;
    }
    qsort(inspected, monkey_count, sizeof(long long), cmp);
    for (int i = 0; i < monkey_count; i++)
    {
        printf("Monkey %d inspected items %lld times.\n", i, inspected[i]);
    }

    printf("Value: %lld\n", inspected[0] * inspected[1]);
}

int main()
{
    FILE *fp;

    fp = fopen("input", "r");
    if (!fp)
        return -1;

    char *lines[10000];
    int rows = 0;
    size_t len = 0;

    // read all the lines first
    while ((getline(&lines[rows++], &len, fp)) != -1)
    {
    }
    rows--;

    // parse the input
    monkey monkeys[100];
    int monkey_count = 0;
    for (int i = 0; i < rows; i++)
    {
        // this is a monkey id line
        // Monkey [X]:
        monkeys[monkey_count].inspected = 0;
        i++;

        // next is items list line
        strsep(&lines[i], ":");
        char *item = lines[i];
        int item_count = 0;
        while ((item = strsep(&lines[i], ",\n")) != NULL)
        {
            monkeys[monkey_count].items[monkeys[monkey_count].items_count++] = atoll(item);
        }
        monkeys[monkey_count].items_count--;
        i++;

        // next is operation
        if (lines[i][23] == '+')
        {
            monkeys[monkey_count].operation = addition;
            monkeys[monkey_count].operation_arg = atoll(lines[i] + 24);
        }
        else if (lines[i][23] == '*')
        {
            if (lines[i][25] == 'o')
            {
                monkeys[monkey_count].operation = square;
            }
            else
            {
                monkeys[monkey_count].operation = multiplication;
                monkeys[monkey_count].operation_arg = atoll(lines[i] + 24);
            }
        }
        i++;

        // next is a test
        monkeys[monkey_count].divisible_by_test = atoll(lines[i] + 21);
        i++;

        // if true
        monkeys[monkey_count].if_true = atoi(lines[i] + 29);
        i++;

        // if false
        monkeys[monkey_count].if_false = atoi(lines[i] + 30);
        i++;

        // empty line
        monkey_count++;
    }

    simulate(monkeys, monkey_count);

    fclose(fp);

    return 0;
}