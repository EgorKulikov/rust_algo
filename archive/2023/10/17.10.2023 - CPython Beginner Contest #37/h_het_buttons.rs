//{"name":"H. HetButtons","group":"CPython.uz - CPython Beginner Contest #37","url":"https://cpython.uz/competitions/contests/contest/312/problem/H","interactive":false,"timeLimit":500,"tests":[{"input":"<het-datatable    #table    (filterButtonClick)=\"openFilter()\"    (gridReady)=\"reload()\"    (onSelectionChanged)=\"changeSelection($event)\"    [columnDefs]=\"columnDefs\"    [defaultColumnDef]=\"defaultColumnDef\"    [request]=\"request\">  <het-button    (btnClick)=\"add()\"    label=\"FORM.ADD\">  </het-button>  <het-button    (btnClick)=\"edit(this.selectedRowId)\"    [disabled]=\"selectedRowData.actStatus === actStatus.CLOSED\"    [label]=\"'FORM.EDIT'\">  </het-button>  <het-button    (btnClick)=\"remove(this.selectedRowId)\"    [disabled]=\"selectedRowData.actStatus === actStatus.DELETED\"    [label]=\"'FORM.DELETE'\">  </het-button></het-datatable>\n","output":"<het-datatable    #table    (filterButtonClick)=\"openFilter()\"    (gridReady)=\"reload()\"    (onSelectionChanged)=\"changeSelection($event)\"    [columnDefs]=\"columnDefs\"    [defaultColumnDef]=\"defaultColumnDef\"    [request]=\"request\">  <het-button    (btnClick)=\"add()\"    [actionControl]=\"'add'\">  </het-button>  <het-button    (btnClick)=\"edit(this.selectedRowId)\"    [disabled]=\"selectedRowData.actStatus === actStatus.CLOSED\"    [actionControl]=\"'edit'\">  </het-button>  <het-button    (btnClick)=\"remove(this.selectedRowId)\"    [disabled]=\"selectedRowData.actStatus === actStatus.DELETED\"    [actionControl]=\"'delete'\">  </het-button></het-datatable>\n"},{"input":"<div class=\"flex flex-col flex-auto min-w-0 p-6 pt-1\" [label]=\"'TITLE'\">    <het-title [title]=\"title\" label=\"TITLE\"></het-title>    <div class=\"h-full\">        <het-datatable #table                       (gridReady)=\"reload()\"                       [request]=\"request\"                       (onSelectionChanged)=\"changeSelection($event)\"                       [defaultColumnDef]=\"defaultColumnDef\"                       (filterButtonClick)=\"openFilter()\"                       [columnDefs]=\"columnDefs\">                <het-button (btnClick)=\"addUpdate()\" [label]=\"'FORM.ADD'\"></het-button>                <het-button (btnClick)=\"addUpdate(this.selectedRowId)\" [disabled]=\"selectedRowId === null\" [label]=\"'FORM.EDIT'\"></het-button>                <het-button (btnClick)=\"removeAct(this.selectedRowId)\" [disabled]=\"selectedRowId === null\" [label]=\"'FORM.DELETE'\"></het-button>                <het-button [disabled]=\"selectedRowId === null\" [label]=\"'ACCOUNTING_SETTLEMENT_LC.ACT_VIOLATION.PRINT_ALL'\"></het-button>                <het-button [disabled]=\"selectedRowId === null\" [label]=\"'ADMINISTRATION.PROTOCOL'\"></het-button>        </het-datatable>    </div></div>\n","output":"<div class=\"flex flex-col flex-auto min-w-0 p-6 pt-1\" [label]=\"'TITLE'\">    <het-title [title]=\"title\" label=\"TITLE\"></het-title>    <div class=\"h-full\">        <het-datatable #table                       (gridReady)=\"reload()\"                       [request]=\"request\"                       (onSelectionChanged)=\"changeSelection($event)\"                       [defaultColumnDef]=\"defaultColumnDef\"                       (filterButtonClick)=\"openFilter()\"                       [columnDefs]=\"columnDefs\">                <het-button (btnClick)=\"addUpdate()\" [actionControl]=\"'add'\"></het-button>                <het-button (btnClick)=\"addUpdate(this.selectedRowId)\" [disabled]=\"selectedRowId === null\" [actionControl]=\"'edit'\"></het-button>                <het-button (btnClick)=\"removeAct(this.selectedRowId)\" [disabled]=\"selectedRowId === null\" [actionControl]=\"'delete'\"></het-button>                <het-button [disabled]=\"selectedRowId === null\" [actionControl]=\"'print_all'\"'\"></het-button>                <het-button [disabled]=\"selectedRowId === null\" [actionControl]=\"'protocol'\"></het-button>        </het-datatable>    </div></div>\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HHetButtons"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::io::Read;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut s = String::new();
    input.read_to_string(&mut s).unwrap();

    let mut s = s.as_str();
    while let Some(pos) = s.find("<het-button") {
        let (before, after) = s.split_at(pos);
        out.print(before);
        s = after;
        let p1 = s.find("label=\"");
        let p2 = s.find("[label]=\"'");
        if p1.is_some() && (p2.is_none() || p1.unwrap() < p2.unwrap()) {
            let (before, after) = s.split_at(p1.unwrap());
            out.print(before);
            s = after;
            let pos = s.find("\"");
            let (_, after) = s.split_at(pos.unwrap() + 1);
            s = after;
            let pos = s.find("\"");
            let (label, after) = s.split_at(pos.unwrap());
            let label = label.split(".").last().unwrap();
            out.print(format!("[actionControl]=\"'{}'", label.to_lowercase()));
            s = after;
        } else {
            let (before, after) = s.split_at(p2.unwrap());
            out.print(before);
            s = after;
            let pos = s.find("\"'");
            let (_, after) = s.split_at(pos.unwrap() + 2);
            s = after;
            let pos = s.find("'\"");
            let (label, after) = s.split_at(pos.unwrap());
            let label = label.split(".").last().unwrap();
            out.print(format!("[actionControl]=\"'{}", label.to_lowercase()));
            s = after;
        }
    }
    out.print(s);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
