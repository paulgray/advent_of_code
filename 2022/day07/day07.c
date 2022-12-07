#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct dir_tree
{
    struct dir_tree *parent;
    char *name;
    int is_dir;
    int size;
    int children_count;
    struct dir_tree **children;
} dir_tree;

dir_tree *parse(char **cmds, int cmds_count, dir_tree *tree)
{
    int i = 0;
    while (i < cmds_count)
    {
        char *cmd = cmds[i];
        i++;

        // we assume the first characater is always '$'
        strsep(&cmd, " ");
        if (cmd[0] == 'l' && cmd[1] == 's')
        {
            char *children[1000];
            int children_count = 0;
            while (i < cmds_count && cmds[i][0] != '$')
            {
                children[children_count] = cmds[i];
                children_count++;
                i++;
            }

            tree->children_count = children_count;
            tree->children = malloc(children_count * sizeof(dir_tree *));

            for (int j = 0; j < children_count; j++)
            {
                dir_tree *child_node = malloc(sizeof(dir_tree));
                child_node->parent = tree;
                child_node->children_count = 0;
                child_node->children = NULL;
                tree->children[j] = child_node;

                // it's a directory
                if (children[j][0] == 'd' && children[j][1] == 'i' && children[j][2] == 'r')
                {
                    child_node->is_dir = 1;
                    child_node->size = 0;
                    child_node->name = children[j] + 4;
                }
                // it's a file
                else
                {
                    child_node->is_dir = 0;
                    child_node->size = atoi(strsep(&children[j], " "));
                    child_node->name = children[j];
                }
            }
        }
        else if (cmd[0] == 'c' && cmd[1] == 'd')
        {
            // skip the 'cd' command, advance to the target dir name
            strsep(&cmd, " ");

            if (cmd[0] == '/')
            {
                // we have to move all the way up
                while (tree->parent)
                    tree = tree->parent;
            }
            else if (cmd[0] == '.' && cmd[1] == '.')
            {
                // move 1 level up
                tree = tree->parent;
            }
            else
            {
                // the 'cmd' is now a directory name
                for (int j = 0; j < tree->children_count; j++)
                {
                    dir_tree *child = tree->children[j];
                    if (strcmp(cmd, child->name) == 0)
                    {
                        tree = child;
                        break;
                    }
                }
            }
        }
    }
    return tree;
}

void compute_sizes(dir_tree *tree)
{
    for (int i = 0; i < tree->children_count; i++)
    {
        // recursively compute sizes of all children
        compute_sizes(tree->children[i]);

        tree->size += tree->children[i]->size;
    }
}

int find_small_dirs(dir_tree *tree, int cap)
{
    int size = 0;
    if (tree->is_dir && tree->size <= cap)
        size += tree->size;

    for (int i = 0; i < tree->children_count; i++)
    {
        size += find_small_dirs(tree->children[i], cap);
    }

    return size;
}

int main()
{
    FILE *fp;

    fp = fopen("input", "r");
    if (!fp)
        return -1;

    char *cmds[10000];
    int cmds_count = 0;

    // read all the commands first
    char *line = NULL;
    size_t len = 0;
    while ((getline(&line, &len, fp)) != -1)
    {
        cmds[cmds_count++] = strdup(line);
    }

    dir_tree *tree = malloc(sizeof(dir_tree));
    dir_tree *top = tree;
    tree->parent = NULL;
    tree->name = "/";
    tree->is_dir = 1;
    tree->size = 0;
    tree->children_count = 0;
    tree->children = NULL;
    parse(cmds, cmds_count, tree);

    compute_sizes(top);

    int cap = 100000;
    int sum_sizes = find_small_dirs(top, cap);
    printf("Sum: %d\n", sum_sizes);

    fclose(fp);
    if (line)
    {
        free(line);
    }

    return 0;
}