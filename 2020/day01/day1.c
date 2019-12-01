#include <stdio.h>
#include <stdlib.h>

int get_fuel(int mass) {
    int sum = (int)(mass / 3) - 2;
    int fuel = sum;

    while((fuel = ((int)(fuel / 3) - 2)) > 0) {
        sum += fuel;
    }

    return sum;
}

int main() {
    FILE *file;
    char *line = NULL;
    size_t len = 0;
    size_t read;
    int sum = 0;

    file = fopen("/tmp/input", "r");
    if(file) {
        while((read = getline(&line, &len, file)) != -1) {
            int mass = 0;

            if(sscanf(line, "%d", &mass) != 1) {
                printf("Failure to read mass from line %s", line);
                return -1;
            }

            sum += get_fuel(mass);
        }

        printf("Sum: %d\n", sum);

        fclose(file);
        if(line) {
            free(line);
        }

        return 0;
    }

    return 1;
}
