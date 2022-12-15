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

int banned_beacon_positions(coord sensors[SIZE], coord beacons[SIZE], int count, int row)
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
        // let's add the full range
        range banned_range;
        banned_range.start = sensors[i].x - distance + delta;
        banned_range.end = sensors[i].x + distance - delta;
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

    // and now remove beacon points
    int removed[100];
    int removed_count = 0;
    for (int i = 0; i < count; i++)
    {
        int skip = 0;
        for (int j = 0; j < removed_count; j++)
        {
            if (beacons[i].x == removed[j])
            {
                skip = 1;
                break;
            }
        }

        if (beacons[i].y == row && skip == 0)
        {
            for (int j = 0; j < range_count; j++)
            {
                if (banned[i].start == -666 && banned[i].end == -666)
                    continue;

                if (banned[j].start <= beacons[i].x && banned[j].end >= beacons[i].x)
                {
                    removed[removed_count] = beacons[i].x;
                    removed_count++;
                    banned_count--;
                }
            }
        }
    }

    return banned_count;
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

    int row = 2000000;
    int pos = banned_beacon_positions(sensors, beacons, count, row);
    printf("Banned beacon positions in row %d: %d\n", row, pos);

    fclose(fp);

    return 0;
}