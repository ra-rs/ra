#[doc = "Register `PL1CTRL1` reader"]
pub struct R(crate::R<PL1CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PL1CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PL1CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PL1CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PL1CTRL1` writer"]
pub struct W(crate::W<PL1CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PL1CTRL1_SPEC>;
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
impl From<crate::W<PL1CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PL1CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L1RESPEN` reader - L1 Response Enable"]
pub type L1RESPEN_R = crate::BitReader<L1RESPEN_A>;
#[doc = "L1 Response Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1RESPEN_A {
    #[doc = "0: LPM is not supported."]
    _0 = 0,
    #[doc = "1: LPM is supported."]
    _1 = 1,
}
impl From<L1RESPEN_A> for bool {
    #[inline(always)]
    fn from(variant: L1RESPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl L1RESPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L1RESPEN_A {
        match self.bits {
            false => L1RESPEN_A::_0,
            true => L1RESPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L1RESPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L1RESPEN_A::_1
    }
}
#[doc = "Field `L1RESPEN` writer - L1 Response Enable"]
pub type L1RESPEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, PL1CTRL1_SPEC, L1RESPEN_A, O>;
impl<'a, const O: u8> L1RESPEN_W<'a, O> {
    #[doc = "LPM is not supported."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(L1RESPEN_A::_0)
    }
    #[doc = "LPM is supported."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(L1RESPEN_A::_1)
    }
}
#[doc = "Field `L1RESPMD` reader - L1 Response Mode"]
pub type L1RESPMD_R = crate::FieldReader<u8, L1RESPMD_A>;
#[doc = "L1 Response Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum L1RESPMD_A {
    #[doc = "0: NYET"]
    _00 = 0,
    #[doc = "1: ACK"]
    _01 = 1,
    #[doc = "2: STALL"]
    _10 = 2,
    #[doc = "3: According to the L1NEGOMD bit"]
    _11 = 3,
}
impl From<L1RESPMD_A> for u8 {
    #[inline(always)]
    fn from(variant: L1RESPMD_A) -> Self {
        variant as _
    }
}
impl L1RESPMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L1RESPMD_A {
        match self.bits {
            0 => L1RESPMD_A::_00,
            1 => L1RESPMD_A::_01,
            2 => L1RESPMD_A::_10,
            3 => L1RESPMD_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == L1RESPMD_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == L1RESPMD_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == L1RESPMD_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == L1RESPMD_A::_11
    }
}
#[doc = "Field `L1RESPMD` writer - L1 Response Mode"]
pub type L1RESPMD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, PL1CTRL1_SPEC, u8, L1RESPMD_A, 2, O>;
impl<'a, const O: u8> L1RESPMD_W<'a, O> {
    #[doc = "NYET"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(L1RESPMD_A::_00)
    }
    #[doc = "ACK"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(L1RESPMD_A::_01)
    }
    #[doc = "STALL"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(L1RESPMD_A::_10)
    }
    #[doc = "According to the L1NEGOMD bit"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(L1RESPMD_A::_11)
    }
}
#[doc = "Field `L1NEGOMD` reader - L1 Response Negotiation Control.NOTE: This bit is valid only when the L1RESPMD\\[1:0\\]
value is 2'b11."]
pub type L1NEGOMD_R = crate::BitReader<L1NEGOMD_A>;
#[doc = "L1 Response Negotiation Control.NOTE: This bit is valid only when the L1RESPMD\\[1:0\\]
value is 2'b11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1NEGOMD_A {
    #[doc = "0: When receive HIRD is larger than HIRDTHR\\[3:0\\], ACK response is returned. In other cases (including HIRD = HIRDTHR\\[3:0\\]), NYET response is returned."]
    _0 = 0,
    #[doc = "1: When receive HIRD is smaller than HIRDTHR\\[3:0\\], ACK response is returned. In other cases (including HIRD = HIRDTHR\\[3:0\\]), NYET response is returned."]
    _1 = 1,
}
impl From<L1NEGOMD_A> for bool {
    #[inline(always)]
    fn from(variant: L1NEGOMD_A) -> Self {
        variant as u8 != 0
    }
}
impl L1NEGOMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L1NEGOMD_A {
        match self.bits {
            false => L1NEGOMD_A::_0,
            true => L1NEGOMD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L1NEGOMD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L1NEGOMD_A::_1
    }
}
#[doc = "Field `L1NEGOMD` writer - L1 Response Negotiation Control.NOTE: This bit is valid only when the L1RESPMD\\[1:0\\]
value is 2'b11."]
pub type L1NEGOMD_W<'a, const O: u8> = crate::BitWriter<'a, u16, PL1CTRL1_SPEC, L1NEGOMD_A, O>;
impl<'a, const O: u8> L1NEGOMD_W<'a, O> {
    #[doc = "When receive HIRD is larger than HIRDTHR\\[3:0\\], ACK response is returned. In other cases (including HIRD = HIRDTHR\\[3:0\\]), NYET response is returned."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(L1NEGOMD_A::_0)
    }
    #[doc = "When receive HIRD is smaller than HIRDTHR\\[3:0\\], ACK response is returned. In other cases (including HIRD = HIRDTHR\\[3:0\\]), NYET response is returned."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(L1NEGOMD_A::_1)
    }
}
#[doc = "Field `DVSQ` reader - DVSQ Extension.DVSQ\\[3\\]
is Mirror of DVSQ\\[2:0\\]
in INTSTS0.Indicates the L1 state together with the device state bits DVSQ\\[2:0\\]."]
pub type DVSQ_R = crate::FieldReader<u8, DVSQ_A>;
#[doc = "DVSQ Extension.DVSQ\\[3\\]
is Mirror of DVSQ\\[2:0\\]
in INTSTS0.Indicates the L1 state together with the device state bits DVSQ\\[2:0\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DVSQ_A {
    #[doc = "0: Powered state"]
    _0000 = 0,
    #[doc = "1: Default state"]
    _0001 = 1,
    #[doc = "2: Address state"]
    _0010 = 2,
    #[doc = "3: Configured state"]
    _0011 = 3,
    #[doc = "4: Suspended state"]
    _0100 = 4,
    #[doc = "5: Suspended state"]
    _0101 = 5,
    #[doc = "6: Suspended state"]
    _0110 = 6,
    #[doc = "7: Suspended state"]
    _0111 = 7,
    #[doc = "8: L1 state"]
    _1000 = 8,
    #[doc = "9: L1 state"]
    _1001 = 9,
    #[doc = "10: L1 state"]
    _1010 = 10,
    #[doc = "11: L1 state"]
    _1011 = 11,
}
impl From<DVSQ_A> for u8 {
    #[inline(always)]
    fn from(variant: DVSQ_A) -> Self {
        variant as _
    }
}
impl DVSQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DVSQ_A> {
        match self.bits {
            0 => Some(DVSQ_A::_0000),
            1 => Some(DVSQ_A::_0001),
            2 => Some(DVSQ_A::_0010),
            3 => Some(DVSQ_A::_0011),
            4 => Some(DVSQ_A::_0100),
            5 => Some(DVSQ_A::_0101),
            6 => Some(DVSQ_A::_0110),
            7 => Some(DVSQ_A::_0111),
            8 => Some(DVSQ_A::_1000),
            9 => Some(DVSQ_A::_1001),
            10 => Some(DVSQ_A::_1010),
            11 => Some(DVSQ_A::_1011),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == DVSQ_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == DVSQ_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == DVSQ_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == DVSQ_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == DVSQ_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == DVSQ_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == DVSQ_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == DVSQ_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == DVSQ_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == DVSQ_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == DVSQ_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == DVSQ_A::_1011
    }
}
#[doc = "Field `HIRDTHR` reader - L1 Response Negotiation Threshold ValueHIRD threshold value used for L1NEGOMD.The format is the same as the HIRD field in HL1CTRL."]
pub type HIRDTHR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HIRDTHR` writer - L1 Response Negotiation Threshold ValueHIRD threshold value used for L1NEGOMD.The format is the same as the HIRD field in HL1CTRL."]
pub type HIRDTHR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PL1CTRL1_SPEC, u8, u8, 4, O>;
#[doc = "Field `L1EXTMD` reader - PHY Control Mode at L1 Return"]
pub type L1EXTMD_R = crate::BitReader<L1EXTMD_A>;
#[doc = "PHY Control Mode at L1 Return\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1EXTMD_A {
    #[doc = "0: SUSPENDM is not set by hardware when Host K is received."]
    _0 = 0,
    #[doc = "1: SUSPENDM is set by hardware when Host K is received."]
    _1 = 1,
}
impl From<L1EXTMD_A> for bool {
    #[inline(always)]
    fn from(variant: L1EXTMD_A) -> Self {
        variant as u8 != 0
    }
}
impl L1EXTMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L1EXTMD_A {
        match self.bits {
            false => L1EXTMD_A::_0,
            true => L1EXTMD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L1EXTMD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L1EXTMD_A::_1
    }
}
#[doc = "Field `L1EXTMD` writer - PHY Control Mode at L1 Return"]
pub type L1EXTMD_W<'a, const O: u8> = crate::BitWriter<'a, u16, PL1CTRL1_SPEC, L1EXTMD_A, O>;
impl<'a, const O: u8> L1EXTMD_W<'a, O> {
    #[doc = "SUSPENDM is not set by hardware when Host K is received."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(L1EXTMD_A::_0)
    }
    #[doc = "SUSPENDM is set by hardware when Host K is received."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(L1EXTMD_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - L1 Response Enable"]
    #[inline(always)]
    pub fn l1respen(&self) -> L1RESPEN_R {
        L1RESPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - L1 Response Mode"]
    #[inline(always)]
    pub fn l1respmd(&self) -> L1RESPMD_R {
        L1RESPMD_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - L1 Response Negotiation Control.NOTE: This bit is valid only when the L1RESPMD\\[1:0\\]
value is 2'b11."]
    #[inline(always)]
    pub fn l1negomd(&self) -> L1NEGOMD_R {
        L1NEGOMD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - DVSQ Extension.DVSQ\\[3\\]
is Mirror of DVSQ\\[2:0\\]
in INTSTS0.Indicates the L1 state together with the device state bits DVSQ\\[2:0\\]."]
    #[inline(always)]
    pub fn dvsq(&self) -> DVSQ_R {
        DVSQ_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - L1 Response Negotiation Threshold ValueHIRD threshold value used for L1NEGOMD.The format is the same as the HIRD field in HL1CTRL."]
    #[inline(always)]
    pub fn hirdthr(&self) -> HIRDTHR_R {
        HIRDTHR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - PHY Control Mode at L1 Return"]
    #[inline(always)]
    pub fn l1extmd(&self) -> L1EXTMD_R {
        L1EXTMD_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - L1 Response Enable"]
    #[inline(always)]
    #[must_use]
    pub fn l1respen(&mut self) -> L1RESPEN_W<0> {
        L1RESPEN_W::new(self)
    }
    #[doc = "Bits 1:2 - L1 Response Mode"]
    #[inline(always)]
    #[must_use]
    pub fn l1respmd(&mut self) -> L1RESPMD_W<1> {
        L1RESPMD_W::new(self)
    }
    #[doc = "Bit 3 - L1 Response Negotiation Control.NOTE: This bit is valid only when the L1RESPMD\\[1:0\\]
value is 2'b11."]
    #[inline(always)]
    #[must_use]
    pub fn l1negomd(&mut self) -> L1NEGOMD_W<3> {
        L1NEGOMD_W::new(self)
    }
    #[doc = "Bits 8:11 - L1 Response Negotiation Threshold ValueHIRD threshold value used for L1NEGOMD.The format is the same as the HIRD field in HL1CTRL."]
    #[inline(always)]
    #[must_use]
    pub fn hirdthr(&mut self) -> HIRDTHR_W<8> {
        HIRDTHR_W::new(self)
    }
    #[doc = "Bit 14 - PHY Control Mode at L1 Return"]
    #[inline(always)]
    #[must_use]
    pub fn l1extmd(&mut self) -> L1EXTMD_W<14> {
        L1EXTMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Function L1 Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pl1ctrl1](index.html) module"]
pub struct PL1CTRL1_SPEC;
impl crate::RegisterSpec for PL1CTRL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pl1ctrl1::R](R) reader structure"]
impl crate::Readable for PL1CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pl1ctrl1::W](W) writer structure"]
impl crate::Writable for PL1CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PL1CTRL1 to value 0"]
impl crate::Resettable for PL1CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
