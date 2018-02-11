//
//  RealtimeConnection.swift
//  AR Chess
//
//  Created by Alex Taffe on 2/11/18.
//  Copyright © 2018 Alex Taffe. All rights reserved.
//

import UIKit
import Socket

class RealTimeConnection: NSObject {
    private var socket:Socket?
    
    override init() {
        do{
            self.socket = try Socket.create()
            try self.socket!.connect(to: "129.21.92.166", port: 4243, timeout: 0)
            print("Connected")
            while(true){
                let result = try self.socket!.readString()
                print("\(String(describing: result))")
            }
        }
        catch{
            
        }
    }
}
