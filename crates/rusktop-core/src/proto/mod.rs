pub mod user {
    pub mod service {
        pub mod v1 {
            tonic::include_proto!("user.service.v1");
        }
    }
}

pub mod rusktop {
    pub mod service {
        pub mod v1 {
            tonic::include_proto!("rusktop.service.v1");
        }
    }
}
