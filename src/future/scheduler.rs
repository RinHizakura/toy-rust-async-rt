/* This is our simple solution to support multiple asynchronous routine. We
 * can achieve by creating each routine as a task which is a futrue, and bind
 * them to the same scheduler which is also a future. So once the scheduler
 * is awaited, it will poll each underlying task and wake any if it is ready. */

struct Scheduler {}
