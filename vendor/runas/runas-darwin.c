#include <errno.h>
#include <unistd.h>
#include <fcntl.h>
#include <Security/Authorization.h>
#include <sys/wait.h>

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wincompatible-pointer-types-discards-qualifiers"


int rust_darwin_gui_runas(const char *prog, const char **argv)
{
    AuthorizationRef authref = 0;
    FILE *pipe = 0;

    // consider locking this, this would leak a bit of memory if called
    // concurrently really quickly
    if (AuthorizationCreate(0, kAuthorizationEmptyEnvironment,
                            kAuthorizationFlagDefaults,
                            &authref) != errAuthorizationSuccess) {
        return -1;
    }

    if (AuthorizationExecuteWithPrivileges(
            authref, prog, kAuthorizationFlagDefaults, argv, &pipe)
            != errAuthorizationSuccess) {
        AuthorizationFree(authref, kAuthorizationFlagDestroyRights);
        return -1;
    }

    int pid = fcntl(fileno(pipe), F_GETOWN, 0);
    int r, status;
    do {
        r = waitpid(pid, &status, 0);
    } while (r == -1 && errno == EINTR);

    AuthorizationFree(authref, kAuthorizationFlagDestroyRights);
    return status;
}

#pragma clang diagnostic pop
