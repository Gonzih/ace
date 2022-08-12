declare namespace Deno.core {
    function opSync(s: string, rid: number, ...args: any[]): void | number;
}

declare namespace global {
    const ace__state_rid: number;
}

interface RuntimeObject {
    tick: () => void,
}

class AceState {
    rid: number;
    entities: RuntimeObject[];

    constructor(rid: number) {
        this.rid = rid;
        this.entities = [];
    }
}

const __state = new AceState(global.ace__state_rid);

enum Component {
    Clickable = "v1_op_insert_clickable",
    Hoverable = "v1_op_insert_hoverable",
    Draggable = "v1_op_insert_draggable",
}

class EntityWrapper {
    eid: number;

    constructor(eid: number) {
        this.eid = eid;
    }

    insert(component: Component): EntityWrapper {
        if (component in Object.keys(Component)) {
            const opName = Component[component];
            Deno.core.opSync(opName, __state.rid, this.eid);
        } else {
            throw "Could not figure out which component to insert in entity";
        }

        return this;
    }
}

class ACE {
    log(s: string): void {
        Deno.core.opSync("v1_op_log", __state.rid, s);
    }

    move_by(x: number, y: number, z: number): void {
        Deno.core.opSync("v1_op_move_by", __state.rid, x, y, z);
    }

    spawn(): EntityWrapper {
        const eid =  Deno.core.opSync("v1_op_spawn", __state.rid);
        if (typeof eid === "number") {
            return new EntityWrapper(eid);
        } else {
            throw "Return type from OP wasn't number";
        }
    }

    __invoke(eid: number, method: string): void {
        if (__state.entities[eid] !== undefined &&
            __state.entities[eid][method] !== undefined) {
            __state.entities[eid][method]();
        }
    }

    __register(eid: number, rt: RuntimeObject): void {
        __state.entities[eid] = rt;
    }
}


const ace = new ACE();
