use tomorrow_core::Result;

pub trait Request {}
pub trait Response {}

pub trait Recuperator<Req, Res> where Req: Request, Res: Response {
    fn compute(&self, request: Req) -> Result<Res>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestRequest {
        expected: bool
    }

    impl TestRequest {
        pub fn new(expected: bool) -> Self {
            TestRequest {
                expected: expected
            }
        }
    }

    impl Request for TestRequest {}

    struct TestResponse {
        actual: bool
    }

    impl TestResponse {
        pub fn new(actual: bool) -> Self {
            TestResponse {
                actual: actual
            }
        }
    }

    impl Response for TestResponse {}

    struct TestRecuperator {}

    impl TestRecuperator {
        pub fn new() -> Self {
            TestRecuperator{}
        }
    }

    impl Recuperator<TestRequest, TestResponse> for TestRecuperator {
        fn compute(&self, request: TestRequest) -> Result<TestResponse> {            
            Ok(TestResponse::new(request.expected))
        }
    }

    #[test]
    fn recuperator_should_return_ok_with_true() {
        let request = TestRequest::new(true);
        let recuperator = TestRecuperator::new();
        let response = recuperator.compute(request);

        assert!(response.is_ok());
        assert!(response.unwrap().actual);
    }

    #[test]
    fn recuperator_should_return_ok_with_false() {
        let request = TestRequest::new(false);
        let recuperator = TestRecuperator::new();
        let response = recuperator.compute(request);

        assert!(response.is_ok());
        assert!(!response.unwrap().actual);
    }
}
