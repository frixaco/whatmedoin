#include <CoreGraphics/CoreGraphics.h>
#include <ApplicationServices/ApplicationServices.h>

__attribute__((visibility("default")))
const char *
get_foreground_app_mac()
{
    static char buffer[256];
    CFArrayRef windowList = CGWindowListCopyWindowInfo(kCGWindowListOptionOnScreenOnly | kCGWindowListExcludeDesktopElements, kCGNullWindowID);

    if (windowList == NULL)
    {
        strcpy(buffer, "Error: Unable to get window list");
        return buffer;
    }

    CFIndex count = CFArrayGetCount(windowList);
    for (CFIndex i = 0; i < count; i++)
    {
        CFDictionaryRef window = CFArrayGetValueAtIndex(windowList, i);
        CFNumberRef layerRef = CFDictionaryGetValue(window, kCGWindowLayer);
        int layer;
        CFNumberGetValue(layerRef, kCFNumberIntType, &layer);

        if (layer == 0)
        { // Main window layer
            CFStringRef ownerName = CFDictionaryGetValue(window, kCGWindowOwnerName);
            if (ownerName && CFStringGetCString(ownerName, buffer, sizeof(buffer), kCFStringEncodingUTF8))
            {
                CFRelease(windowList);
                return buffer;
            }
        }
    }

    CFRelease(windowList);
    strcpy(buffer, "Unknown");
    return buffer;
}