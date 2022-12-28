#[doc = "Register `VBTOCTLR` reader"]
pub struct R(crate::R<VBTOCTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VBTOCTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VBTOCTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VBTOCTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VBTOCTLR` writer"]
pub struct W(crate::W<VBTOCTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VBTOCTLR_SPEC>;
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
impl From<crate::W<VBTOCTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VBTOCTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VCH0OEN` reader - VBATT Wakeup I/O 0 Output Enable"]
pub type VCH0OEN_R = crate::BitReader<VCH0OEN_A>;
#[doc = "VBATT Wakeup I/O 0 Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH0OEN_A {
    #[doc = "0: VBATWIO0 output disabled"]
    _0 = 0,
    #[doc = "1: VBATWIO0 output enabled"]
    _1 = 1,
}
impl From<VCH0OEN_A> for bool {
    #[inline(always)]
    fn from(variant: VCH0OEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VCH0OEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCH0OEN_A {
        match self.bits {
            false => VCH0OEN_A::_0,
            true => VCH0OEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH0OEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH0OEN_A::_1
    }
}
#[doc = "Field `VCH0OEN` writer - VBATT Wakeup I/O 0 Output Enable"]
pub type VCH0OEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, VBTOCTLR_SPEC, VCH0OEN_A, O>;
impl<'a, const O: u8> VCH0OEN_W<'a, O> {
    #[doc = "VBATWIO0 output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VCH0OEN_A::_0)
    }
    #[doc = "VBATWIO0 output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VCH0OEN_A::_1)
    }
}
#[doc = "Field `VCH1OEN` reader - VBATT Wakeup I/O 1 Output Enable"]
pub type VCH1OEN_R = crate::BitReader<VCH1OEN_A>;
#[doc = "VBATT Wakeup I/O 1 Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH1OEN_A {
    #[doc = "0: VBATWIO1 output disabled"]
    _0 = 0,
    #[doc = "1: VBATWIO1 output enabled"]
    _1 = 1,
}
impl From<VCH1OEN_A> for bool {
    #[inline(always)]
    fn from(variant: VCH1OEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VCH1OEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCH1OEN_A {
        match self.bits {
            false => VCH1OEN_A::_0,
            true => VCH1OEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH1OEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH1OEN_A::_1
    }
}
#[doc = "Field `VCH1OEN` writer - VBATT Wakeup I/O 1 Output Enable"]
pub type VCH1OEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, VBTOCTLR_SPEC, VCH1OEN_A, O>;
impl<'a, const O: u8> VCH1OEN_W<'a, O> {
    #[doc = "VBATWIO1 output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VCH1OEN_A::_0)
    }
    #[doc = "VBATWIO1 output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VCH1OEN_A::_1)
    }
}
#[doc = "Field `VCH2OEN` reader - VBATT Wakeup I/O 2 Output Enable"]
pub type VCH2OEN_R = crate::BitReader<VCH2OEN_A>;
#[doc = "VBATT Wakeup I/O 2 Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH2OEN_A {
    #[doc = "0: VBATWIO2 output disabled"]
    _0 = 0,
    #[doc = "1: VBATWIO2 output enabled"]
    _1 = 1,
}
impl From<VCH2OEN_A> for bool {
    #[inline(always)]
    fn from(variant: VCH2OEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VCH2OEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCH2OEN_A {
        match self.bits {
            false => VCH2OEN_A::_0,
            true => VCH2OEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH2OEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH2OEN_A::_1
    }
}
#[doc = "Field `VCH2OEN` writer - VBATT Wakeup I/O 2 Output Enable"]
pub type VCH2OEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, VBTOCTLR_SPEC, VCH2OEN_A, O>;
impl<'a, const O: u8> VCH2OEN_W<'a, O> {
    #[doc = "VBATWIO2 output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VCH2OEN_A::_0)
    }
    #[doc = "VBATWIO2 output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VCH2OEN_A::_1)
    }
}
#[doc = "Field `VOUT0LSEL` reader - VBATT Wakeup I/O 0 Output Level Selection"]
pub type VOUT0LSEL_R = crate::BitReader<VOUT0LSEL_A>;
#[doc = "VBATT Wakeup I/O 0 Output Level Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VOUT0LSEL_A {
    #[doc = "0: Output L before VBATT wakeup trigger"]
    _0 = 0,
    #[doc = "1: Output H before VBATT wakeup trigger"]
    _1 = 1,
}
impl From<VOUT0LSEL_A> for bool {
    #[inline(always)]
    fn from(variant: VOUT0LSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl VOUT0LSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VOUT0LSEL_A {
        match self.bits {
            false => VOUT0LSEL_A::_0,
            true => VOUT0LSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VOUT0LSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VOUT0LSEL_A::_1
    }
}
#[doc = "Field `VOUT0LSEL` writer - VBATT Wakeup I/O 0 Output Level Selection"]
pub type VOUT0LSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, VBTOCTLR_SPEC, VOUT0LSEL_A, O>;
impl<'a, const O: u8> VOUT0LSEL_W<'a, O> {
    #[doc = "Output L before VBATT wakeup trigger"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VOUT0LSEL_A::_0)
    }
    #[doc = "Output H before VBATT wakeup trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VOUT0LSEL_A::_1)
    }
}
#[doc = "Field `VCOU1LSEL` reader - VBATT Wakeup I/O 1 Output Level Selection"]
pub type VCOU1LSEL_R = crate::BitReader<VCOU1LSEL_A>;
#[doc = "VBATT Wakeup I/O 1 Output Level Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCOU1LSEL_A {
    #[doc = "0: Output L before VBATT wake up trigger"]
    _0 = 0,
    #[doc = "1: Output H before VBATT wake up trigger"]
    _1 = 1,
}
impl From<VCOU1LSEL_A> for bool {
    #[inline(always)]
    fn from(variant: VCOU1LSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl VCOU1LSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCOU1LSEL_A {
        match self.bits {
            false => VCOU1LSEL_A::_0,
            true => VCOU1LSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCOU1LSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCOU1LSEL_A::_1
    }
}
#[doc = "Field `VCOU1LSEL` writer - VBATT Wakeup I/O 1 Output Level Selection"]
pub type VCOU1LSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, VBTOCTLR_SPEC, VCOU1LSEL_A, O>;
impl<'a, const O: u8> VCOU1LSEL_W<'a, O> {
    #[doc = "Output L before VBATT wake up trigger"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VCOU1LSEL_A::_0)
    }
    #[doc = "Output H before VBATT wake up trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VCOU1LSEL_A::_1)
    }
}
#[doc = "Field `VOUT2LSEL` reader - VBATT Wakeup I/O 2 Output Level Selection"]
pub type VOUT2LSEL_R = crate::BitReader<VOUT2LSEL_A>;
#[doc = "VBATT Wakeup I/O 2 Output Level Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VOUT2LSEL_A {
    #[doc = "0: Output L before VBATT wake up trigger"]
    _0 = 0,
    #[doc = "1: Output H before VBATT wake up trigger"]
    _1 = 1,
}
impl From<VOUT2LSEL_A> for bool {
    #[inline(always)]
    fn from(variant: VOUT2LSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl VOUT2LSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VOUT2LSEL_A {
        match self.bits {
            false => VOUT2LSEL_A::_0,
            true => VOUT2LSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VOUT2LSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VOUT2LSEL_A::_1
    }
}
#[doc = "Field `VOUT2LSEL` writer - VBATT Wakeup I/O 2 Output Level Selection"]
pub type VOUT2LSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, VBTOCTLR_SPEC, VOUT2LSEL_A, O>;
impl<'a, const O: u8> VOUT2LSEL_W<'a, O> {
    #[doc = "Output L before VBATT wake up trigger"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VOUT2LSEL_A::_0)
    }
    #[doc = "Output H before VBATT wake up trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VOUT2LSEL_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - VBATT Wakeup I/O 0 Output Enable"]
    #[inline(always)]
    pub fn vch0oen(&self) -> VCH0OEN_R {
        VCH0OEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBATT Wakeup I/O 1 Output Enable"]
    #[inline(always)]
    pub fn vch1oen(&self) -> VCH1OEN_R {
        VCH1OEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VBATT Wakeup I/O 2 Output Enable"]
    #[inline(always)]
    pub fn vch2oen(&self) -> VCH2OEN_R {
        VCH2OEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VBATT Wakeup I/O 0 Output Level Selection"]
    #[inline(always)]
    pub fn vout0lsel(&self) -> VOUT0LSEL_R {
        VOUT0LSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VBATT Wakeup I/O 1 Output Level Selection"]
    #[inline(always)]
    pub fn vcou1lsel(&self) -> VCOU1LSEL_R {
        VCOU1LSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VBATT Wakeup I/O 2 Output Level Selection"]
    #[inline(always)]
    pub fn vout2lsel(&self) -> VOUT2LSEL_R {
        VOUT2LSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBATT Wakeup I/O 0 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vch0oen(&mut self) -> VCH0OEN_W<0> {
        VCH0OEN_W::new(self)
    }
    #[doc = "Bit 1 - VBATT Wakeup I/O 1 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vch1oen(&mut self) -> VCH1OEN_W<1> {
        VCH1OEN_W::new(self)
    }
    #[doc = "Bit 2 - VBATT Wakeup I/O 2 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vch2oen(&mut self) -> VCH2OEN_W<2> {
        VCH2OEN_W::new(self)
    }
    #[doc = "Bit 3 - VBATT Wakeup I/O 0 Output Level Selection"]
    #[inline(always)]
    #[must_use]
    pub fn vout0lsel(&mut self) -> VOUT0LSEL_W<3> {
        VOUT0LSEL_W::new(self)
    }
    #[doc = "Bit 4 - VBATT Wakeup I/O 1 Output Level Selection"]
    #[inline(always)]
    #[must_use]
    pub fn vcou1lsel(&mut self) -> VCOU1LSEL_W<4> {
        VCOU1LSEL_W::new(self)
    }
    #[doc = "Bit 5 - VBATT Wakeup I/O 2 Output Level Selection"]
    #[inline(always)]
    #[must_use]
    pub fn vout2lsel(&mut self) -> VOUT2LSEL_W<5> {
        VOUT2LSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VBATT Output Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vbtoctlr](index.html) module"]
pub struct VBTOCTLR_SPEC;
impl crate::RegisterSpec for VBTOCTLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vbtoctlr::R](R) reader structure"]
impl crate::Readable for VBTOCTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vbtoctlr::W](W) writer structure"]
impl crate::Writable for VBTOCTLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VBTOCTLR to value 0"]
impl crate::Resettable for VBTOCTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
