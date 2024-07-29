// 4. Get User Profile
//    GET /api/users/me
//    Input:  No body, requires valid access token in Authorization header
//    Output: { "id": string, "username": string, "email": string, "createdAt": string, "lastLogin": string }
//
// 5. Update User Profile
//    PUT /api/users/me
//    Input:  { "username": string, "email": string }, requires valid access token in Authorization header
//    Output: { "id": string, "username": string, "email": string, "updatedAt": string }
//
//
// 10. Get User by ID (Admin only)
//     GET /api/users/{userId}
//     Input:  User ID in URL parameter, requires valid admin access token in Authorization header
//     Output: { "id": string, "username": string, "email": string, "createdAt": string, "lastLogin": string, "status": string }
