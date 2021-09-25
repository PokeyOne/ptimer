#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>

typedef struct {
  int hours;
  int minutes;
  int seconds;
  int status;
} processed_args_t;

int str_length(char* str) {
  int count = 0;

  while(str[count] != '\0') {
    count++;
  }

  return count;
}

int str_start_with(char* str, const char* start) {
  int str_len = str_length(str);
  int start_len = str_length(start);

  if(str_len < start_len) {
    return 0;
  }

  for(int i = 0; i < start_len; i++) {
    if(str[i] != start[i]) {
      return 0;
    }
  }

  return 1;
}

int str_equal_to(char* a, char* b) {
  int a_len = str_length(a);
  int b_len = str_length(b);

  if (a_len != b_len) {
    return 0;
  }

  for(int i = 0; i < a_len; i++) {
    if (a[i] != b[i]) {
      return 0;
    }
  }

  return 1;
}

processed_args_t process_args(int argc, char* argv[]) {
  processed_args_t result;
  result.seconds = 0;
  result.minutes = 0;
  result.hours = 0;
  result.status = 0;

  if(argc < 2) {
    result.status = 1;
    return result;
  }

  for(int i = 1; i < argc; i++) {
    if(str_start_with(argv[i], "--")) {
      if(str_equal_to(argv[i], "--hours")) {
        if(i < argc-1) {
          result.hours = atoi(argv[i+1]);
          i++;
        } else {
          fprintf(stderr, "Expected number after '--hours'\n");
          result.status = 1;
          return result;
        }
      } else if(str_equal_to(argv[i], "--minutes")) {
        if(i < argc-1) {
          result.minutes = atoi(argv[i+1]);
          i++;
        } else {
          fprintf(stderr, "Expected number after '--minutes'\n");
          result.status = 1;
          return result;
        }
      } else if(str_equal_to(argv[i], "--seconds")) {
        if(i < argc-1) {
          result.seconds = atoi(argv[i+1]);
          i++;
        } else {
          fprintf(stderr, "Expected number after '--seconds'\n");
          result.status = 1;
          return result;
        }
      } else {
        fprintf(stderr, "Unexpected double-dash option '%s'\n", argv[i]);
        result.status = 1;
        return result;
      }
    } else if(str_start_with(argv[i], "-")) {
      if(str_equal_to(argv[i], "-h")) {
        if(i < argc-1) {
          result.hours = atoi(argv[i+1]);
          i++;
        } else {
          fprintf(stderr, "Expected number after '-h'\n");
          result.status = 1;
          return result;
        }
      } else if(str_equal_to(argv[i], "-m")) {
        if(i < argc-1) {
          result.minutes = atoi(argv[i+1]);
          i++;
        } else {
          fprintf(stderr, "Expected number after '-m'\n");
          result.status = 1;
          return result;
        }
      } else if(str_equal_to(argv[i], "-s")) {
        if(i < argc-1) {
          result.seconds = atoi(argv[i+1]);
          i++;
        } else {
          fprintf(stderr, "Expected number after '-s'\n");
          result.status = 1;
          return result;
        }
      } else {
        fprintf(stderr, "Unexpected single-dash option '%s'\n", argv[i]);
        result.status = 1;
        return result;
      }
    } else {
      // Here should only handle a single integer and then there should be no
      // more arguments
      result.seconds = atoi(argv[i]);
      break;
    }
  }

  // Balance the args
  result.minutes += (result.seconds - (result.seconds % 60)) / 60;
  result.seconds %= 60;
  result.hours += (result.minutes - (result.minutes % 60)) / 60;
  result.minutes %= 60;

  return result;
}

int main(int argc, char* argv[]) {
  processed_args_t pargs = process_args(argc, argv);

  if(pargs.status != 0) {
    printf("Could not process arguments\n");
    return 1;
  } else {
    printf("Starting timer for %02dh %02dm %02ds\n", pargs.hours, pargs.minutes, pargs.seconds);
  }

  int total_seconds = pargs.seconds + pargs.minutes * 60 + pargs.hours * 60 * 60;
  time_t start_time = time(0);

  // Hide the cursor
  printf("\e[?25l");

  int seconds_left;
  while((seconds_left = total_seconds - (time(0) - start_time)) >= 0) {
    int minutes = (seconds_left - seconds_left%60) / 60;
    int seconds = seconds_left%60;
    printf("\r%02dm %ds      ", minutes, seconds);
    usleep(500);
  }
  printf("\n");

  // Bring back the cursor
  printf("\e[?25h");

  return 0;
}
