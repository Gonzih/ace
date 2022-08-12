(function(eid) {
    {{{ code }}}

    const rt = {
        tick: tick || function(){},
    }

    ace.__register(eid, rt);
})({{ eid }});
