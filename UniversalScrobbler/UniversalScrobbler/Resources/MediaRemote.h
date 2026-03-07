#import <Foundation/Foundation.h>

extern NSString *kMRMediaRemoteNowPlayingInfoDidChangeNotification;

extern CFDictionaryRef MRMediaRemoteGetNowPlayingInfo(void);

extern void MRMediaRemoteRegisterForNowPlayingNotifications(dispatch_queue_t queue);

extern void MRMediaRemoteUnregisterForNowPlayingNotifications(void);
