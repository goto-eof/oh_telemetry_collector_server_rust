use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct TelemetryGenericResponse<T> {
    pub success: bool,
    pub result: T,
}
