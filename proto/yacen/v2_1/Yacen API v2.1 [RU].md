
# Yacen API v2.1

## Аутентификация

Все RPC используют заголовки gRPC metadata для сквозной целостности и аутентичности:

-   **pubkey**: публичный ключ Ed25519 длиной 32 байта, закодированный в Base64
    
-   **signature**: подпись Ed25519, закодированная в Base64, вычисленная по сериализованному сообщению protobuf
    

----------

## MessageUniqueID

Используется в каждом сообщении для обеспечения свежести и предотвращения повторных атак (replay attacks).

-   `nonce` (bytes[24])
    
-   `timestamp` (`google.protobuf.Timestamp`)
    

----------

## Сообщения

### CreateRoom

**CreateRoomReq**

-   `public_info` (`RoomInfoPublic`)
    
-   `muid` (`MessageUniqueID`)
    

**CreateRoomRes**

-   `room_id` (hex-кодированные bytes[32])
    
-   `muid` (`MessageUniqueID`)
    

----------

### RequestJoinRoom

**RequestJoinRoomReq**

-   `room_id` (hex-кодированные bytes[32])
    
-   `ecies_public_key` (публичный ключ ECIES, закодированный в Base64)
    
-   `muid` (`MessageUniqueID`)
    

**RequestJoinRoomRes**

-   `join_request_id` (строка)
    
-   `muid` (`MessageUniqueID`)
    

----------

### ApproveJoinRequest

**ApproveJoinRequestReq**

-   `join_request_id` (строка)
    
-   `ecies_encrypted_room_key` (закодированный в Base64)
    
-   `muid` (`MessageUniqueID`)
    

**ApproveJoinRequestRes**

-   `muid` (`MessageUniqueID`)
    

----------

### GetJoinRequestStatus

**GetJoinRequestStatusReq**

-   `join_request_id` (строка)
    
-   `muid` (`MessageUniqueID`)
    

**GetJoinRequestStatusRes**

-   `status` (`JoinRequestStatus`: DENIED = 0, PENDING = 1, APPROVED = 2)
    
-   `ecies_encrypted_room_key` (закодированный в Base64; только если статус = APPROVED)
    
-   `muid` (`MessageUniqueID`)
    

----------

### GetRoomInfo

**GetRoomInfoReq**

-   `room_id` (hex-кодированные bytes[32])
    
-   `muid` (`MessageUniqueID`)
    

**GetRoomInfoRes**

-   `public_info` (`RoomInfoPublic`)
    
-   `private_info` (`RoomInfoPrivate`, необязательно)
    
-   `muid` (`MessageUniqueID`)
    

----------

### UpdateRoomInfo

**UpdateRoomInfoReq**

-   `room_id` (hex-кодированные bytes[32])
    
-   `public_info` (`RoomInfoPublic`)
    
-   `muid` (`MessageUniqueID`)
    

**UpdateRoomInfoRes**

-   `muid` (`MessageUniqueID`)
    

----------

### DeleteRoom

**DeleteRoomReq**

-   `room_id` (hex-кодированные bytes[32])
    
-   `muid` (`MessageUniqueID`)
    

**DeleteRoomRes**

-   `muid` (`MessageUniqueID`)
    

----------

### GiveExtendedRights

**GiveExtendedRightsReq**

-   `target_pubkeys` (повторяющиеся публичные ключи длиной 32 байта, закодированные в Base64)
    
-   `muid` (`MessageUniqueID`)
    

**GiveExtendedRightsRes**

-   `muid` (`MessageUniqueID`)
    

----------

### RemoveExtendedRights

**RemoveExtendedRightsReq**

-   `target_pubkeys` (повторяющиеся публичные ключи длиной 32 байта, закодированные в Base64)
    
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

-   `room_id` (hex-кодированные bytes[32])
    
-   `message` (`Message`)
    
-   `signature` (`Ed25519Signature`)
    

**SendMessageRes**

-   `muid` (`MessageUniqueID`)
    

----------

### GetMessages

**GetMessageReq**

-   `room_id` (hex-кодированные bytes[32])
    
-   `muid` (`MessageUniqueID`)
    

**GetMessageRes**

-   `message` (`Message`)
    
-   `signature` (`Ed25519Signature`)
    
-   `muid` (`MessageUniqueID`)
    

_(ответ в виде стрима)_

----------

### Передача файлов

**FileChunk**

-   `chunk_id` (int32)
    
-   `chunk_count` (int32)
    
-   `encrypted_data` (bytes)
    
-   `muid` (`MessageUniqueID`)
    

#### UploadFile

**UploadFileReq**

-   `room_id` (hex-кодированные bytes[32])
    
-   `chunk` (`FileChunk`)
    
-   `chunk_signature` (`Ed25519Signature`)
    

**UploadFileRes**

-   `file_id` (hex-кодированные bytes[32])
    
-   `expires_at` (`google.protobuf.Timestamp`)
    
-   `muid` (`MessageUniqueID`)
    

#### DownloadFile

**DownloadFileReq**

-   `file_id` (hex-кодированные bytes[32])
    
-   `muid` (`MessageUniqueID`)
    

**DownloadFileRes**

-   `chunk` (`FileChunk`)
    
-   `chunk_signature` (`Ed25519Signature`)
    
-   `muid` (`MessageUniqueID`)
    

_(ответ в виде стрима)_

----------

## Вспомогательные типы

### RoomInfoPublic

-   `encrypted_room_name` (bytes)
    
-   `encrypted_room_description` (bytes)
    
-   `room_type` (`RoomType`: COMMON = 0, CHANNEL = 1)
    

### RoomInfoPrivate

-   `extended_rights_public_keys` (повторяющиеся публичные ключи длиной 32 байта, закодированные в Base64)
    
-   `allowed_public_keys` (повторяющиеся публичные ключи длиной 32 байта, закодированные в Base64)
    
-   `pending_join_requests` (повторяющиеся строки)
    

### Ed25519Signature

-   `signature` (bytes[64])
    
-   `public_key` (публичный ключ длиной 32 байта, закодированный в Base64)
    

### Перечисления (Enums)

-   **RoomType**: `ROOM_TYPE_COMMON` = 0, `ROOM_TYPE_CHANNEL` = 1
    
-   **JoinRequestStatus**: `DENIED` = 0, `PENDING` = 1, `APPROVED` = 2
    

----------

**Синтаксис:** `proto3`  
**Пакет:** `yacen_api.v2_1`  
**Импорты:** `google/protobuf/timestamp.proto`
