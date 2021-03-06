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
CGEventFlags flags = kCGEventFlagMaskShift;
CGEventRef ev;
CGEventSourceRef source = CGEventSourceCreate (kCGEventSourceStateCombinedSessionState);

//press down            
ev = CGEventCreateKeyboardEvent (source, letter, true);    
CGEventSetFlags(ev,flags | CGEventGetFlags(ev)); //combine flags                        
CGEventPost(kCGHIDEventTap,ev);
CFRelease(ev);              

//press up                                  
ev = CGEventCreateKeyboardEvent (source, letter, false);                       
CGEventSetFlags(ev,flags | CGEventGetFlags(ev)); //combine flags                        
CGEventPost(kCGHIDEventTap,ev); 
CFRelease(ev);              
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
