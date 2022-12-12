#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef enum cmd
{
    noop,
    addx,
} cmd;

typedef struct instr
{
    cmd command;
    int arg;
} instr;

typedef struct exec_env
{
    int ip;
    int r_x;
    instr cmds[10000];
} exec_env;

void execute(exec_env *env)
{
    int result = 0;
    for (env->ip = 1; env->ip < 221; env->ip++)
    {
        if ((env->ip % 40) == 20)
        {
            //  printf("####\nCycle %03d\tx = %d\tsignal strength: %d\n####\n", env->ip, env->r_x, env->ip * env->r_x);
            result += env->ip * env->r_x;
        }

        //        printf("%03d\t%d(%d)\tx: %d\n", env->ip, env->cmds[env->ip].command, env->cmds[env->ip].arg, env->r_x);

        if (env->cmds[env->ip].command == noop)
        {
        }
        else if (env->cmds[env->ip].command == addx)
        {
            env->r_x += env->cmds[env->ip].arg;
        }
    }

    printf("Result: %d\n", result);
}

int main()
{
    FILE *fp;

    fp = fopen("input", "r");
    if (!fp)
        return -1;

    char *line = NULL;
    size_t len = 0;
    exec_env env;
    env.ip = 1;
    env.r_x = 1;
    while ((getline(&line, &len, fp)) != -1)
    {
        char *c = strsep(&line, " \n");
        instr ins;
        if (strcmp(c, "noop") == 0)
        {
            ins.command = noop;
            env.cmds[env.ip++] = ins;
        }
        else if (strcmp(c, "addx") == 0)
        {
            ins.command = noop;
            ins.arg = 0;
            env.cmds[env.ip++] = ins;

            ins.command = addx;
            ins.arg = atoi(line);
            env.cmds[env.ip++] = ins;
        }

        line = NULL;
    }
    env.ip = 1;
    execute(&env);

    fclose(fp);

    return 0;
}