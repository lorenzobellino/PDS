#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <stdlib.h>
#include <string.h>

typedef struct
{
    int type;
    float val;
    long timestamp;
} ValueStruct;

typedef struct
{
    int type;
    float val[10];
    long timestamp;
} MValueStruct;

typedef struct
{
    int type;
    char message[21]; // stringa null terminated lung max 20
} MessageStruct;

typedef struct
{
    int type;
    union
    {
        ValueStruct val;
        MValueStruct mvals;
        MessageStruct messages;
    };
} ExportData;

static float *getRandomArray(int n);

static char *rand_string(char *str, size_t size);

void export(ExportData *d, int n, FILE *fp);

void printData(ExportData *d, int n)
{
    for (int i = 0; i < n; i++)
    {
        switch (d[i].type)
        {
        case 0:
            printf("ValueStruct: %f, %ld\n", d[i].val.val, d[i].val.timestamp);
            break;
        case 1:
            printf("MValueStruct: ");
            for (int j = 0; j < 10; j++)
            {
                printf("%f, ", d[i].mvals.val[j]);
            }
            printf("%ld\n", d[i].mvals.timestamp);
            break;
        case 2:
            printf("MessageStruct: %s\n", d[i].messages.message);
            break;
        default:
            break;
        } 
    }
    printf("\n");
}


int main(int argc, char *argv[])
{
    FILE *fp;
    int type, i;
    int n = 100;
    char str[21];
    ExportData *data;
    ValueStruct vs;
    MessageStruct ms;
    MValueStruct mvs;
    data = (ExportData *)malloc(n * sizeof(ExportData));
    srand(time(NULL));
    for (i = 0; i < n; i++)
    {
        type = rand() % 3;
        data[i].type = type;
        switch (type)
        {
        case 0:
            //fprintf(stderr,"ValueStruct\n");
            // vs = {type, rand() % 100, time(NULL)};
            vs.type = type;
            vs.timestamp = time(NULL);
            vs.val = (float) (rand() % 100) / 7;
            data[i].val = vs;
            break;
        case 1:
            //fprintf(stderr, "MValueStruct\n");
            // float *v = getRandomArray(10);
            // mvs = {type, *getRandomArray(10), time(NULL)};
            mvs.type = type;
            mvs.timestamp = time(NULL);
            memcpy(mvs.val, getRandomArray(10), 10*sizeof(float));
            // mvs.val = getRandomArray(10);
            data[i].mvals = mvs;
            // MValueStruct mvs = {type, v, time(NULL)};
            break;
        case 2:
            //fprintf(stderr,"MessageStruct\n");
            // ms = {type, rand_string(str, 20)};
            ms.type = type;
            strcpy(ms.message , rand_string(str, 20));
            data[i].messages = ms;
            // fprintf(stdout,"stringa in data : %s\n", data[i].messages.message);
            // fprintf(stdout, "string in ms : %s\n", ms.message);
            // printf("ssstr : %s\n", rand_string(str, 20));
            // scanf("%d", &type);
            break;
        default:
            break;
        }
        
    }
    
    // printData(data, n);
    fp = fopen("export.bin", "wb");
    export(data, n, fp);

    fprintf(stdout, "sizeof export %ld\n", sizeof(ExportData));
    fprintf(stdout, "sizeof ValueStruct %ld\n", sizeof(ValueStruct));
    fprintf(stdout, "sizeof MValueStruct %ld\n", sizeof(MValueStruct));
    fprintf(stdout, "sizeof MessageStruct %ld\n", sizeof(MessageStruct));
    fprintf(stdout, "sizeof int %ld\n", sizeof(int));
    fprintf(stdout, "sizeof float %ld\n", sizeof(float));
    fprintf(stdout, "sizeof long %ld\n", sizeof(long));
    fprintf(stdout, "sizeof char %ld\n", sizeof(char));
    fclose(fp);
    
    return 0;
}

void export(ExportData *d, int n, FILE *fp)
{
    fwrite(d, sizeof(ExportData), n, fp);
}

static char *rand_string(char *str, size_t size)
{
    // int k;
    const char charset[] = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    if (size) {
        --size;
        for (size_t n = 0; n < size; n++) {
            int key = rand() % (int) (sizeof charset - 1);
            str[n] = charset[key];
        }
        str[size] = '\0';
    }
    // printf("stringa : %s\n", str);
    return str;
}

float *getRandomArray(int n)
{
    float num,den,v;
    float *arr = (float *)malloc(n * sizeof(float));
    for (int i = 0; i < n; i++)
    {
        num = rand() % 100;
        den = ((rand() % 10) + i+1);
        // v = rand() % 100 ;
        // / ((rand() % 10) + i+1);
        v = num / den;
        // fprintf(stdout,"n / d = v : %f / %f = %f\n", num,den,v);
        arr[i] = v;
    }
    return arr;
}

