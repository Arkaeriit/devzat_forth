#include <pthread.h>

// For some reason, the rust runtime makes threads not cancellable by default.
// This function can be used by any threads to render it cancellable.
void cancellable_thread(void) {
	int old_state;
	pthread_setcancelstate(PTHREAD_CANCEL_ENABLE, &old_state);
	pthread_setcanceltype(PTHREAD_CANCEL_ASYNCHRONOUS, &old_state);
}

