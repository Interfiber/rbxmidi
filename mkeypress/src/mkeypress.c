#include "mkeypress.h"
#include <Carbon/Carbon.h>
#include <ApplicationServices/ApplicationServices.h>
#include <stdio.h>
#include <string.h>
#define array_count(a) (sizeof((a)) / sizeof(*(a)))

void test(){
    printf("Test!!");
}

int has_ax_access(){
    const void *keys[] = { kAXTrustedCheckOptionPrompt };
    const void *values[] = { kCFBooleanTrue };
    CFDictionaryRef options = CFDictionaryCreate(NULL, keys, values, array_count(keys), &kCFCopyStringDictionaryKeyCallBacks, &kCFTypeDictionaryValueCallBacks);
    bool result = AXIsProcessTrustedWithOptions(options);
    CFRelease(options);
    return result; 
}

void press_key(int key){
    // Create a HID hardware event source
    CGEventSourceRef src = CGEventSourceCreate(kCGEventSourceStateHIDSystemState);

    // Create a new keyboard press event
    CGEventRef evt = CGEventCreateKeyboardEvent(src, (CGKeyCode) key, true);

    // Post the keyboard event, then release the key
    CGEventPost (kCGHIDEventTap, evt);
    CFRelease (evt); CFRelease (src);
    usleep(2);
}

void press_shift_key(int letter){
    CGEventRef event1, e5;
    event1 = CGEventCreateKeyboardEvent(NULL, (CGKeyCode)letter, true);//'z' keydown event
    CGEventSetFlags(event1, kCGEventFlagMaskShift);//set shift key down for above event
    CGEventPost(kCGSessionEventTap, event1);//post event
    e5 = CGEventCreateKeyboardEvent(NULL, (CGKeyCode)kVK_Shift, false);
    CGEventPost(kCGSessionEventTap, e5);
    CFRelease(event1);
    CFRelease(e5);
}

void release_key(int key){
      // Create an HID hardware event source
    CGEventSourceRef src = CGEventSourceCreate(kCGEventSourceStateHIDSystemState);
    // Create a new keyboard key release event
    CGEventRef evt = CGEventCreateKeyboardEvent(src, (CGKeyCode) key, false);

    // Post keyboard event and release
    CGEventPost (kCGHIDEventTap, evt);
    CFRelease (evt); CFRelease (src);

    usleep(2);
}

void sendkey(int letter, bool uppercase){
    if (has_ax_access()){
        // send key events
        if (uppercase){
            press_shift_key(letter);
        } else {
            press_key(letter);
            release_key(letter);
        }
    }
}