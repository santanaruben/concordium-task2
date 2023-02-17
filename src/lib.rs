//! # A Concordium V1 smart contract
use concordium_std::*;
use core::fmt::Debug;

/// Your smart contract state.
#[derive(Serialize, SchemaType, Clone)]
pub struct State {
    operand1: u32,
    operand2: u32,
    operation: String,
    result: u32,
}

#[derive(Serialize, SchemaType)]
struct InitParameter{
    operand1: u32,
    operand2: u32,
    operation: String,
}

#[init(contract = "calculator")]
fn init<S: HasStateApi>(
    _ctx: &impl HasInitContext,
    _state_builder: &mut StateBuilder<S>,
) -> InitResult<State> {
    //let param: InitParameter = _ctx.parameter_cursor().get()?;
    Ok(State {
       operand1: 2,
       operand2: 3,
       operation: "+".to_string(),
       result: 0,
    })
}

/// Your smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serial, SchemaType)]
enum ContractError {
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParamsError,
    DivisionError,
    OperationError,
}

#[receive(
    contract = "calculator",
    name = "calculate",
    parameter = "InitParameter",
    error = "ContractError",
    mutable,
)]
fn calculate<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &mut impl HasHost<State, StateApiType = S>,
) -> Result<(), ContractError> {
    let param: InitParameter = _ctx.parameter_cursor().get()?;
    let state = _host.state_mut();
    if param.operation == "/".to_string() {
        ensure!(param.operand2 != 0, ContractError::DivisionError)
    }
    let mut result = 0;
    if param.operation == "+".to_string(){
        result = param.operand1+param.operand2;
    }     
    else if param.operation == "-".to_string(){
            result = param.operand1-param.operand2;
    }
    else if param.operation == "*".to_string(){
        result = param.operand1*param.operand2;
    }
    else if param.operation == "/".to_string(){
        result = param.operand1/param.operand2;
    }
    else { 
        ensure!(false, ContractError::OperationError);
    }
    state.operand1 = param.operand1;
    state.operand2 = param.operand2;
    state.operation = param.operation;
    state.result = result;
    Ok(())
}

/// View function that returns the content of the state.
#[derive(Serialize, SchemaType)]
struct CalculateView {
    equation: String,
    result: u32,
}
#[receive(contract = "calculator", name = "view", return_value = "CalculateView")]
fn view<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<CalculateView> {
    let state = host.state();
    let _a = state.operand1.clone().to_string();
    let _b = state.operation.clone();
    let _c = state.operand2.clone().to_string();
    let mut equation = format!("{}{}",_a, _b);
    equation = format!("{}{}",equation, _c);
    let result = state.result.clone();
    Ok(CalculateView{
        equation,
        result,
    })
}
/// Test that invoking the `receive` endpoint with the `false` parameter
/// succeeds in updating the contract.
#[concordium_cfg_test]
mod tests {
    use super::*;
    use test_infrastructure::*;

    #[test]
    fn calculate_works(){           
        let mut _ctx = TestReceiveContext::empty();
        let param: InitParameter = InitParameter{
            operand1: 8,
            operand2: 4,
            operation: "+".to_string(),
        };
        let parameter = to_bytes(&param);
        _ctx.set_parameter(&parameter);

        let state: State = State{
            operand1: param.operand1, 
            operand2: param.operand2,
            operation: param.operation,
            result: 0,
        };
        let mut host = TestHost::new(state, TestStateBuilder::new());
        let res = calculate(&_ctx, &mut host);

        assert!(res.is_ok());
        assert_eq!(host.state().result, 12);
        }
    }
