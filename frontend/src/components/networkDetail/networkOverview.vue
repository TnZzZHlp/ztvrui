<script setup lang="ts">
import NetworkNameIdComponent from './overview/networkNameId.vue'
import NetworkCreationTimeComponent from './overview/networkCreateTime.vue'
import NetworkTypeComponent from './overview/networkType.vue'
import NetworkMTUComponent from './overview/networkOther.vue'
import NetworkIPAssignmentPoolsComponent from './overview/networkIPAssignmentPools.vue'
import NetworkIPv4AssignmentComponent from './overview/networkIPv4Assignment.vue'
import NetworkIPv6AssignmentComponent from './overview/networkIPv6Assignment.vue'
import NetworkDNSComponent from './overview/networkDNS.vue'
import NetworkRoutesComponent from './overview/networkRoutes.vue'
import NetworkRulesComponent from './overview/networkRules.vue'
</script>

<template>
  <div class="p-4 sm:grid-cols-1 grid lg:grid-cols-4 gap-4 w-full">
    <NetworkNameIdComponent />
    <NetworkCreationTimeComponent />
    <NetworkTypeComponent />
    <NetworkMTUComponent />
    <NetworkIPAssignmentPoolsComponent />
    <NetworkIPv4AssignmentComponent />
    <NetworkIPv6AssignmentComponent />
    <NetworkRoutesComponent />
    <NetworkDNSComponent />
    <NetworkRulesComponent />
  </div>
</template>
