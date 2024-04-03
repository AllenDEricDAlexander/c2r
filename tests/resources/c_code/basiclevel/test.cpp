#include <pthread.h>
#include <stdlib.h>

int data = 0;
void *increment(void *v) {
    for (int i = 0; i < 1000000; i++) {
        data++;
    }
    return NULL;
}

int main() {
    pthread_t t1, t2;
    pthread_create(&t1, NULL, increment, NULL);
    pthread_create(&t2, NULL, increment, NULL);
    pthread_join(t1, NULL);
    pthread_join(t2, NULL);
}