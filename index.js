
const express = require('express');
const jquery = require('jquery');
const jsdom = require('jsdom');
const fetch = require('node-fetch');
// import express from 'express';
// import jquery from 'jquery';
// import jsdom from 'jsdom';
// import fetch from 'node-fetch';

const app = express();
const port = 8000;
const dom = new jsdom.JSDOM("");
const $ = jquery(dom.window)

app.use(express.static('public'))

app.get('/lfu', async function (req, res) {
    const url = "https://lfuonline.uibk.ac.at/public/lfuonline_lv.home#lv-details";

    const body = $(await request_lfu(url)).find("#content");
    const content = $(body[0].children);

    if (body != null) {
        res.send({ success: true, result: parse_xnode_list(content) });
    } else {
        res.send({ succes: false });
    }
})

app.get('/lfu/:id', async function (req, res) {
    const id = req.params.id;

    const body = $(await request_lfu(create_lfu_url(id)));

    if (body != null) {
        res.send({ success: true, result: parse_xnode_list(body) });
    } else {
        res.send({ succes: false });
    }
})

function create_lfu_url(id) {
    return `https://lfuonline.uibk.ac.at/public/lfuonline_lv.rubrik_ajax?r=${id}&l=0`;
}

function request_lfu(url) {
    console.log(`Requesting: ${url}`);
    return fetch(url)
        .then(function (response) {
            return response.text();
        }).catch(function (error) {
            return null;
        });
}

function parse_xnode_list(elem) {
    return elem.filter('.xnode').map(function() {
        var that = $(this);
        return {
            id: that.attr('data-id'),
            text: get_xnode_text(that)
        };
    }).get();
}

function get_xnode_text(elem) {
    return elem.children().remove().end().text().trim();
}

app.listen(port, () => {
    console.log(`⚡️[server]: Server is running at https://localhost:${port}`);
});
