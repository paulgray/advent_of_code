#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define SIZE 50

typedef struct coord
{
    int x;
    int y;
} coord;

typedef struct range
{
    int start;
    int end;
} range;

int range_cmp(const void *left, const void *right)
{
    range l = *(range *)left;
    range r = *(range *)right;

    if (l.start == r.start)
        return r.end - l.end;

    return l.start - r.start;
}

long long search_for_beacon(coord sensors[SIZE], coord beacons[SIZE], int count, int max)
{
    for (int row = 0; row < max; row++)
    {
        // we won't count individual points - it's too much
        // instead, let's track banned ranges, merge them
        // count the length - and finally remove known beacons
        range banned[1000];
        int range_count = 0;

        // for every sensor
        for (int i = 0; i < count; i++)
        {
            // see if the closest beacon coverage area overlaps with the selected row
            int delta = abs(row - sensors[i].y);
            int distance = abs(sensors[i].x - beacons[i].x) + abs(sensors[i].y - beacons[i].y);

            // this sensor won't affect our measurements
            if (delta > distance)
                continue;

            // the interesting row is in the range, let's see how many coords overlap
            // let's add the full range, but within [0, max]
            range banned_range;
            banned_range.start = sensors[i].x - distance + delta;
            banned_range.start = banned_range.start < 0 ? 0 : banned_range.start;
            banned_range.end = sensors[i].x + distance - delta;
            banned_range.end = banned_range.end > max ? max : banned_range.end;
            banned[range_count] = banned_range;
            range_count++;
        }

        // now let's merge the ranges
        // first - sort them
        qsort(banned, range_count, sizeof(range), range_cmp);

        // then - merge whatever overlaps
        for (int i = 0; i < range_count; i++)
        {
            if (banned[i].start == -666 && banned[i].end == -666)
                continue;

            for (int j = i + 1; j < range_count; j++)
            {
                if (banned[j].start == -666 && banned[j].end == -666)
                    continue;

                if (banned[i].end >= banned[j].start)
                {
                    banned[i].end = banned[i].end > banned[j].end ? banned[i].end : banned[j].end;
                    banned[j].start = -666;
                    banned[j].end = -666;
                }
            }
        }

        // let's count the total length of all ranges
        int banned_count = 0;
        for (int i = 0; i < range_count; i++)
        {
            if (banned[i].start == -666 && banned[i].end == -666)
                continue;

            banned_count += banned[i].end - banned[i].start + 1;
        }

        // see how many possible spots are there
        int possible_spots = max - banned_count + 1;
        if (possible_spots == 1)
        {
            // we found the right row, let's now find the right 'x'
            int x = 0;
            for (x = 0; x <= max; x++)
            {
                // for every banned range - see if 'x' falls within it
                int found = 1;
                for (int i = 0; i < range_count; i++)
                {
                    if (x >= banned[i].start && x <= banned[i].end)
                    {
                        x = banned[i].end;
                        found = 0;
                        break;
                    }
                }

                if (found)
                    break;
            }

            printf("Found a spot: %d, %d\n", x, row);

            // this is going to overflow, so just do the math outside of C
            long long frequency = 4000000 * x + row;
            return frequency;
        }
    }

    return -1;
}

int main()
{
    FILE *fp;

    fp = fopen("input", "r");
    if (!fp)
        return -1;

    char *line = NULL;
    size_t len = 0;

    // read inputs
    coord sensors[SIZE], beacons[SIZE];
    int count = 0;
    while ((getline(&line, &len, fp)) != -1)
    {
        // find sensor coords
        strsep(&line, "=");
        sensors[count].x = atoi(strsep(&line, ","));
        strsep(&line, "=");
        sensors[count].y = atoi(strsep(&line, ":"));

        // find beacon coords
        strsep(&line, "=");
        beacons[count].x = atoi(strsep(&line, ","));
        strsep(&line, "=");
        beacons[count].y = atoi(line);

        count++;
    }

    // int row = 10;
    // int pos = banned_beacon_positions(sensors, beacons, count, row);
    int max = 4000000;
    long long frequency = search_for_beacon(sensors, beacons, count, max);
    printf("Tuning frequency: %lld\n", frequency);

    fclose(fp);

    return 0;
}