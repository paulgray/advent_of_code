#include <stdlib.h>
#include <stdio.h>

const long MAX_POINTS = 7173000;
const int PLAYERS_COUNT = 464;

struct node {
    long value;
    struct node *prev;
    struct node *next;
};

long scores[PLAYERS_COUNT];
long ring_size = 1;
struct node ring;

int current_player = 0;
struct node *current_marble;

void remove_marble() {
    struct node *to_remove = current_marble->prev->prev->prev->prev->prev->prev->prev;
    struct node *before = to_remove->prev;
    struct node *after = to_remove->next;

    scores[current_player] += to_remove->value;
    before->next = after;
    after->prev = before;
    free(to_remove);

    current_marble = after;

    ring_size--;
}

void insert_marble(long turn) {
    struct node *new_marble;
    struct node *before = current_marble->next;
    struct node *after = before->next;

    new_marble = malloc(sizeof(struct node));
    new_marble->value = turn;
    new_marble->next = after;
    new_marble->prev = before;
    before->next = new_marble;
    after->prev = new_marble;

    current_marble = new_marble;

    ring_size++;
}

void print_ring() {
    struct node *ptr = &ring;

    printf("[%d]\t", current_player);
    do {
        if(ptr == current_marble) {
            printf("(%ld)", ptr->value);
        } else {
            printf("%ld", ptr->value);
        }
        printf("\t");

        ptr = ptr->next;
    } while(ptr != &ring);
    printf("\n");
}

int main() {
    for(int i=0; i<PLAYERS_COUNT; i++)
        scores[i] = 0;
    ring.value = 0;
    ring.prev = &ring;
    ring.next = &ring;
    current_marble = &ring;

    for(long turn=1; turn<MAX_POINTS; turn++) {
        if((turn % 23) == 0) {
            scores[current_player] += turn;
            remove_marble();
        } else {
            insert_marble(turn);
        }

        current_player = (current_player+1) % PLAYERS_COUNT;
    }

    long max_score = 0;
    for(int i=0; i<PLAYERS_COUNT; i++) {
        if(scores[i] > max_score)
            max_score = scores[i];
    }
    printf("%ld\n", max_score);
}
