#[doc = "Register `CFDGCFG` reader"]
pub struct R(crate::R<CFDGCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDGCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDGCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDGCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDGCFG` writer"]
pub struct W(crate::W<CFDGCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDGCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CFDGCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDGCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPRI` reader - Transmission Priority"]
pub type TPRI_R = crate::BitReader<TPRI_A>;
#[doc = "Transmission Priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPRI_A {
    #[doc = "0: ID priority"]
    _0 = 0,
    #[doc = "1: Message buffer number priority"]
    _1 = 1,
}
impl From<TPRI_A> for bool {
    #[inline(always)]
    fn from(variant: TPRI_A) -> Self {
        variant as u8 != 0
    }
}
impl TPRI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPRI_A {
        match self.bits {
            false => TPRI_A::_0,
            true => TPRI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TPRI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TPRI_A::_1
    }
}
#[doc = "Field `TPRI` writer - Transmission Priority"]
pub type TPRI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGCFG_SPEC, TPRI_A, O>;
impl<'a, const O: u8> TPRI_W<'a, O> {
    #[doc = "ID priority"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPRI_A::_0)
    }
    #[doc = "Message buffer number priority"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPRI_A::_1)
    }
}
#[doc = "Field `DCE` reader - DLC Check Enable"]
pub type DCE_R = crate::BitReader<DCE_A>;
#[doc = "DLC Check Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCE_A {
    #[doc = "0: DLC check disabled"]
    _0 = 0,
    #[doc = "1: DLC check enabled"]
    _1 = 1,
}
impl From<DCE_A> for bool {
    #[inline(always)]
    fn from(variant: DCE_A) -> Self {
        variant as u8 != 0
    }
}
impl DCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCE_A {
        match self.bits {
            false => DCE_A::_0,
            true => DCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCE_A::_1
    }
}
#[doc = "Field `DCE` writer - DLC Check Enable"]
pub type DCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGCFG_SPEC, DCE_A, O>;
impl<'a, const O: u8> DCE_W<'a, O> {
    #[doc = "DLC check disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCE_A::_0)
    }
    #[doc = "DLC check enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCE_A::_1)
    }
}
#[doc = "Field `DRE` reader - DLC Replacement Enable"]
pub type DRE_R = crate::BitReader<DRE_A>;
#[doc = "DLC Replacement Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRE_A {
    #[doc = "0: DLC replacement disabled"]
    _0 = 0,
    #[doc = "1: DLC replacement enabled"]
    _1 = 1,
}
impl From<DRE_A> for bool {
    #[inline(always)]
    fn from(variant: DRE_A) -> Self {
        variant as u8 != 0
    }
}
impl DRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRE_A {
        match self.bits {
            false => DRE_A::_0,
            true => DRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRE_A::_1
    }
}
#[doc = "Field `DRE` writer - DLC Replacement Enable"]
pub type DRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGCFG_SPEC, DRE_A, O>;
impl<'a, const O: u8> DRE_W<'a, O> {
    #[doc = "DLC replacement disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRE_A::_0)
    }
    #[doc = "DLC replacement enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRE_A::_1)
    }
}
#[doc = "Field `MME` reader - Mirror Mode Enable"]
pub type MME_R = crate::BitReader<MME_A>;
#[doc = "Mirror Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MME_A {
    #[doc = "0: Mirror mode disabled"]
    _0 = 0,
    #[doc = "1: Mirror mode enabled"]
    _1 = 1,
}
impl From<MME_A> for bool {
    #[inline(always)]
    fn from(variant: MME_A) -> Self {
        variant as u8 != 0
    }
}
impl MME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MME_A {
        match self.bits {
            false => MME_A::_0,
            true => MME_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MME_A::_1
    }
}
#[doc = "Field `MME` writer - Mirror Mode Enable"]
pub type MME_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGCFG_SPEC, MME_A, O>;
impl<'a, const O: u8> MME_W<'a, O> {
    #[doc = "Mirror mode disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MME_A::_0)
    }
    #[doc = "Mirror mode enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MME_A::_1)
    }
}
#[doc = "Field `DCS` reader - Data Link Controller Clock Select"]
pub type DCS_R = crate::BitReader<DCS_A>;
#[doc = "Data Link Controller Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCS_A {
    #[doc = "0: Internal clean clock"]
    _0 = 0,
    #[doc = "1: External clock source connected to CANMCLK pin"]
    _1 = 1,
}
impl From<DCS_A> for bool {
    #[inline(always)]
    fn from(variant: DCS_A) -> Self {
        variant as u8 != 0
    }
}
impl DCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCS_A {
        match self.bits {
            false => DCS_A::_0,
            true => DCS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCS_A::_1
    }
}
#[doc = "Field `DCS` writer - Data Link Controller Clock Select"]
pub type DCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGCFG_SPEC, DCS_A, O>;
impl<'a, const O: u8> DCS_W<'a, O> {
    #[doc = "Internal clean clock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCS_A::_0)
    }
    #[doc = "External clock source connected to CANMCLK pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCS_A::_1)
    }
}
#[doc = "Field `CMPOC` reader - CANFD Message Payload Overflow Configuration"]
pub type CMPOC_R = crate::BitReader<CMPOC_A>;
#[doc = "CANFD Message Payload Overflow Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPOC_A {
    #[doc = "0: Message is rejected"]
    _0 = 0,
    #[doc = "1: Message payload is cut to fit to configured message size"]
    _1 = 1,
}
impl From<CMPOC_A> for bool {
    #[inline(always)]
    fn from(variant: CMPOC_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPOC_A {
        match self.bits {
            false => CMPOC_A::_0,
            true => CMPOC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPOC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPOC_A::_1
    }
}
#[doc = "Field `CMPOC` writer - CANFD Message Payload Overflow Configuration"]
pub type CMPOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGCFG_SPEC, CMPOC_A, O>;
impl<'a, const O: u8> CMPOC_W<'a, O> {
    #[doc = "Message is rejected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPOC_A::_0)
    }
    #[doc = "Message payload is cut to fit to configured message size"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPOC_A::_1)
    }
}
#[doc = "Field `TSP` reader - Timestamp Prescaler"]
pub type TSP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSP` writer - Timestamp Prescaler"]
pub type TSP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDGCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `TSSS` reader - Timestamp Source Select"]
pub type TSSS_R = crate::BitReader<TSSS_A>;
#[doc = "Timestamp Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSSS_A {
    #[doc = "0: Source clock for timestamp counter is peripheral clock"]
    _0 = 0,
    #[doc = "1: Source clock for timestamp counter is bit time clock"]
    _1 = 1,
}
impl From<TSSS_A> for bool {
    #[inline(always)]
    fn from(variant: TSSS_A) -> Self {
        variant as u8 != 0
    }
}
impl TSSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSSS_A {
        match self.bits {
            false => TSSS_A::_0,
            true => TSSS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSSS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSSS_A::_1
    }
}
#[doc = "Field `TSSS` writer - Timestamp Source Select"]
pub type TSSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGCFG_SPEC, TSSS_A, O>;
impl<'a, const O: u8> TSSS_W<'a, O> {
    #[doc = "Source clock for timestamp counter is peripheral clock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSSS_A::_0)
    }
    #[doc = "Source clock for timestamp counter is bit time clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSSS_A::_1)
    }
}
#[doc = "Field `ITRCP` reader - Interval Timer Reference Clock Prescaler"]
pub type ITRCP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ITRCP` writer - Interval Timer Reference Clock Prescaler"]
pub type ITRCP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDGCFG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - Transmission Priority"]
    #[inline(always)]
    pub fn tpri(&self) -> TPRI_R {
        TPRI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DLC Check Enable"]
    #[inline(always)]
    pub fn dce(&self) -> DCE_R {
        DCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DLC Replacement Enable"]
    #[inline(always)]
    pub fn dre(&self) -> DRE_R {
        DRE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mirror Mode Enable"]
    #[inline(always)]
    pub fn mme(&self) -> MME_R {
        MME_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Link Controller Clock Select"]
    #[inline(always)]
    pub fn dcs(&self) -> DCS_R {
        DCS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CANFD Message Payload Overflow Configuration"]
    #[inline(always)]
    pub fn cmpoc(&self) -> CMPOC_R {
        CMPOC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Timestamp Prescaler"]
    #[inline(always)]
    pub fn tsp(&self) -> TSP_R {
        TSP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Timestamp Source Select"]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Interval Timer Reference Clock Prescaler"]
    #[inline(always)]
    pub fn itrcp(&self) -> ITRCP_R {
        ITRCP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Transmission Priority"]
    #[inline(always)]
    #[must_use]
    pub fn tpri(&mut self) -> TPRI_W<0> {
        TPRI_W::new(self)
    }
    #[doc = "Bit 1 - DLC Check Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dce(&mut self) -> DCE_W<1> {
        DCE_W::new(self)
    }
    #[doc = "Bit 2 - DLC Replacement Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dre(&mut self) -> DRE_W<2> {
        DRE_W::new(self)
    }
    #[doc = "Bit 3 - Mirror Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mme(&mut self) -> MME_W<3> {
        MME_W::new(self)
    }
    #[doc = "Bit 4 - Data Link Controller Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn dcs(&mut self) -> DCS_W<4> {
        DCS_W::new(self)
    }
    #[doc = "Bit 5 - CANFD Message Payload Overflow Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn cmpoc(&mut self) -> CMPOC_W<5> {
        CMPOC_W::new(self)
    }
    #[doc = "Bits 8:11 - Timestamp Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn tsp(&mut self) -> TSP_W<8> {
        TSP_W::new(self)
    }
    #[doc = "Bit 12 - Timestamp Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn tsss(&mut self) -> TSSS_W<12> {
        TSSS_W::new(self)
    }
    #[doc = "Bits 16:31 - Interval Timer Reference Clock Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn itrcp(&mut self) -> ITRCP_W<16> {
        ITRCP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdgcfg](index.html) module"]
pub struct CFDGCFG_SPEC;
impl crate::RegisterSpec for CFDGCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdgcfg::R](R) reader structure"]
impl crate::Readable for CFDGCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdgcfg::W](W) writer structure"]
impl crate::Writable for CFDGCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDGCFG to value 0"]
impl crate::Resettable for CFDGCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
