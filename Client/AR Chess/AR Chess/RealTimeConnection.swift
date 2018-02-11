//
//  RealtimeConnection.swift
//  AR Chess
//
//  Created by Alex Taffe on 2/11/18.
//  Copyright © 2018 Alex Taffe. All rights reserved.
//

import UIKit
import Socket
import SwiftyJSON

class RealTimeConnection: NSObject {
    private var socket:Socket?
    
    override init() {
        super.init()
        do{
            self.socket = try Socket.create()
            try self.socket!.connect(to: "129.21.92.166", port: 4243, timeout: 0)
            print("Connected")
            while(true){
                let result = try self.socket!.readString()
                if result == nil{
                    continue
                }
                let json = JSON.init(parseJSON: result!)
                self.parseSocketData(data: json)
                
                
            }
        }
        catch{
            
        }
    }
    
    private func parseSocketData(data:JSON){
        
    }
    
    
}
