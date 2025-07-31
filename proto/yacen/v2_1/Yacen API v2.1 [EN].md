
# Yacen API v2.1

## Authentication

All RPCs use gRPC metadata headers for end-to-end integrity and authenticity:

-   **pubkey**: Base64-encoded 32 byte Ed25519 public key
    
-   **signature**: Base64-encoded Ed25519 signature over the serialized protobuf message
    

----------

## MessageUniqueID

Used in every message to ensure freshness and prevent replay attacks.

-   `nonce` (bytes[24])
    
-   `timestamp` (`google.protobuf.Timestamp`)
    

----------

## Messages

### CreateRoom

**CreateRoomReq**

-   `public_info` (`RoomInfoPublic`)
    
-   `muid` (`MessageUniqueID`)
    

**CreateRoomRes**

-   `room_id` (hex-encoded bytes[32])
    
-   `muid` (`MessageUniqueID`)
    

----------

### RequestJoinRoom

**RequestJoinRoomReq**

-   `room_id` (hex-encoded bytes[32])
    
-   `ecies_public_key` (Base64-encoded ECIES public key)
    
-   `muid` (`MessageUniqueID`)
    

**RequestJoinRoomRes**

-   `join_request_id` (string)
    
-   `muid` (`MessageUniqueID`)
    

----------

### ApproveJoinRequest

**ApproveJoinRequestReq**

-   `join_request_id` (string)
    
-   `ecies_encrypted_room_key` (Base64-encoded)
    
-   `muid` (`MessageUniqueID`)
    

**ApproveJoinRequestRes**

-   `muid` (`MessageUniqueID`)
    

----------

### GetJoinRequestStatus

**GetJoinRequestStatusReq**

-   `join_request_id` (string)
    
-   `muid` (`MessageUniqueID`)
    

**GetJoinRequestStatusRes**

-   `status` (`JoinRequestStatus`: DENIED = 0, PENDING = 1, APPROVED = 2)
    
-   `ecies_encrypted_room_key` (Base64-encoded; only if status = APPROVED)
    
-   `muid` (`MessageUniqueID`)
    

----------

### GetRoomInfo

**GetRoomInfoReq**

-   `room_id` (hex-encoded bytes[32])
    
-   `muid` (`MessageUniqueID`)
    

**GetRoomInfoRes**

-   `public_info` (`RoomInfoPublic`)
    
-   `private_info` (`RoomInfoPrivate`, optional)
    
-   `muid` (`MessageUniqueID`)
    

----------

### UpdateRoomInfo

**UpdateRoomInfoReq**

-   `room_id` (hex-encoded bytes[32])
    
-   `public_info` (`RoomInfoPublic`)
    
-   `muid` (`MessageUniqueID`)
    

**UpdateRoomInfoRes**

-   `muid` (`MessageUniqueID`)
    

----------

### DeleteRoom

**DeleteRoomReq**

-   `room_id` (hex-encoded bytes[32])
    
-   `muid` (`MessageUniqueID`)
    

**DeleteRoomRes**

-   `muid` (`MessageUniqueID`)
    

----------

### GiveExtendedRights

**GiveExtendedRightsReq**

-   `target_pubkeys` (repeated Base64-encoded bytes[32])
    
-   `muid` (`MessageUniqueID`)
    

**GiveExtendedRightsRes**

-   `muid` (`MessageUniqueID`)
    

----------

### RemoveExtendedRights

**RemoveExtendedRightsReq**

-   `target_pubkeys` (repeated Base64-encoded bytes[32])
    
-   `muid` (`MessageUniqueID`)
    

**RemoveExtendedRightsRes**

-   `muid` (`MessageUniqueID`)
    

----------

### SendMessage

**Message**

-   `encrypted_text` (bytes)
    
-   `encrypted_file_id` (bytes)
    
-   `muid` (`MessageUniqueID`)
    

**SendMessageReq**

-   `room_id` (hex-encoded bytes[32])
    
-   `message` (`Message`)
    
-   `signature` (`Ed25519Signature`)
    

**SendMessageRes**

-   `muid` (`MessageUniqueID`)
    

----------

### GetMessages

**GetMessageReq**

-   `room_id` (hex-encoded bytes[32])
    
-   `muid` (`MessageUniqueID`)
    

**GetMessageRes**

-   `message` (`Message`)
    
-   `signature` (`Ed25519Signature`)
    
-   `muid` (`MessageUniqueID`)
    

_(streaming response)_

----------

### File Transfer

**FileChunk**

-   `chunk_id` (int32)
    
-   `chunk_count` (int32)
    
-   `encrypted_data` (bytes)
    
-   `muid` (`MessageUniqueID`)
    

#### UploadFile

**UploadFileReq**

-   `room_id` (hex-encoded bytes[32])
    
-   `chunk` (`FileChunk`)
    
-   `chunk_signature` (`Ed25519Signature`)
    

**UploadFileRes**

-   `file_id` (hex-encoded bytes[32])
    
-   `expires_at` (`google.protobuf.Timestamp`)
    
-   `muid` (`MessageUniqueID`)
    

#### DownloadFile

**DownloadFileReq**

-   `file_id` (hex-encoded bytes[32])
    
-   `muid` (`MessageUniqueID`)
    

**DownloadFileRes**

-   `chunk` (`FileChunk`)
    
-   `chunk_signature` (`Ed25519Signature`)
    
-   `muid` (`MessageUniqueID`)
    

_(streaming response)_

----------

## Supporting Types

### RoomInfoPublic

-   `encrypted_room_name` (bytes)
    
-   `encrypted_room_description` (bytes)
    
-   `room_type` (`RoomType`: COMMON = 0, CHANNEL = 1)
    

### RoomInfoPrivate

-   `extended_rights_public_keys` (repeated Base64-encoded bytes[32])
    
-   `allowed_public_keys` (repeated Base64-encoded bytes[32])
    
-   `pending_join_requests` (repeated strings)
    

### Ed25519Signature

-   `signature` (bytes[64])
    
-   `public_key` (Base64-encoded bytes[32])
    

### Enums

-   **RoomType**: `ROOM_TYPE_COMMON` = 0, `ROOM_TYPE_CHANNEL` = 1
    
-   **JoinRequestStatus**: `DENIED` = 0, `PENDING` = 1, `APPROVED` = 2
    

----------

**Syntax:** `proto3`  
**Package:** `yacen_api.v2_1`  
**Imports:** `google/protobuf/timestamp.proto`
