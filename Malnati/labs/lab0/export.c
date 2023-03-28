#include<stdio.h>
#include<stdlib.h>
#include <time.h>
#include <stdlib.h>



typedef struct {
    int type;
    float val;
    long timestamp;
} ValueStruct;

typedef struct {
    int type;
    float val[10];  
    long timestamp;
} MValueStruct;

typedef struct {
    int type;
    char message[21]; // stringa null terminated lung max 20
} MessageStruct;

typedef struct {
    int type;
    union {
        ValueStruct val;
        MValueStruct mvals;
        MessageStruct messages;
};
} ExportData;


int main(int argc, char *argv[])
{
    FILE *fp;
    ExportData d1,d2,d3;
    int i = 0;
    srand(time(NULL));   
    for (i=0;i<3;i++){

        switch(i){
            case 0:
                
        }

        int type = rand();
        float f = (float)rand()/(float)(RAND_MAX/100);
        long t = time(NULL);

    }

    
return 0;
}
